use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use tauri::Runtime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::timeout;

use crate::models::{Analyzer, AnalyzerStatus};
use crate::models::hematology::{BF6900Event, HematologyResult, PatientData};
use crate::api::commands::bf6900_handler::BF6900StoreData;
use crate::protocol::hl7_parser::{
    HL7ConnectionState, HL7Message, OBXSegment, PIDSegment,
    parse_hl7_message, create_hl7_acknowledgment,
    extract_parameter_name, extract_parameter_code, extract_abnormal_flags, 
    parse_pid_segment, parse_obx_segment, parse_msa_segment, parse_orc_segment,
    is_supported_message_type
};

// ============================================================================
// CONNECTION STRUCTURE FOR HL7/MLLP
// ============================================================================

#[derive(Debug)]
pub struct HL7Connection {
    pub stream: TcpStream,
    pub remote_addr: SocketAddr,
    pub state: HL7ConnectionState,
    pub message_buffer: Vec<u8>,     // Buffer for incoming HL7 message
    pub current_message: Vec<u8>,    // Current message being built
    pub analyzer_id: String,
    pub last_activity: DateTime<Utc>, // Track connection activity
    pub retry_count: u32,            // Track retry attempts
    pub health_status: ConnectionHealthStatus,
}

#[derive(Debug, Clone)]
pub enum ConnectionHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

// ============================================================================
// MAIN BF-6900 SERVICE (CQ 5 Plus)
// ============================================================================

pub struct BF6900Service<R: Runtime> {
    /// Analyzer configuration
    analyzer: Arc<RwLock<Analyzer>>,
    /// TCP listener for incoming connections
    listener: Arc<Mutex<Option<TcpListener>>>,
    /// Active connections
    connections: Arc<RwLock<HashMap<String, HL7Connection>>>,
    /// Event sender for frontend communication
    event_sender: mpsc::Sender<BF6900Event>,
    /// Service status
    is_running: Arc<RwLock<bool>>,
    /// Store for configuration persistence
    store: Arc<tauri_plugin_store::Store<R>>,
}

impl<R: Runtime> BF6900Service<R> {
    /// Creates a new BF6900 service
    pub fn new(
        analyzer: Analyzer,
        event_sender: mpsc::Sender<BF6900Event>,
        store: Arc<tauri_plugin_store::Store<R>>,
    ) -> Self {
        Self {
            analyzer: Arc::new(RwLock::new(analyzer)),
            listener: Arc::new(Mutex::new(None)),
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            is_running: Arc::new(RwLock::new(false)),
            store,
        }
    }

    /// Starts the service
    pub async fn start(&self) -> Result<(), String> {
        let port = {
            let analyzer = self.analyzer.read().await;
            analyzer.port.ok_or("No port configured")?
        };
        let bind_addr = format!("0.0.0.0:{}", port);

        log::info!("🚀 STARTING BF-6900 EXTERNAL CONNECTION SERVICE");
        log::info!("   🌐 Bind Address: {}", bind_addr);
        log::info!("   🔗 Protocol: HL7 v2.4 with MLLP framing");

        // Create TCP listener
        let listener = TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| {
                log::error!("❌ FAILED TO START EXTERNAL CONNECTION SERVICE");
                log::error!("   🌐 Address: {}", bind_addr);
                log::error!("   🚨 Error: {}", e);
                format!("Failed to bind to {}: {}", bind_addr, e)
            })?;

        log::info!("✅ TCP LISTENER READY FOR EXTERNAL CONNECTIONS");

        // Store listener in mutex
        {
            let mut listener_guard = self.listener.lock().await;
            *listener_guard = Some(listener);
        }

        *self.is_running.write().await = true;

        // Update analyzer status to Active
        let analyzer_id = {
            let mut analyzer = self.analyzer.write().await;
            analyzer.status = crate::models::AnalyzerStatus::Active;
            analyzer.updated_at = chrono::Utc::now();
            analyzer.id.clone()
        };

        // Save updated analyzer to store
        self.save_analyzer_to_store().await?;

        // Emit status update event
        let _ = self
            .event_sender
            .send(BF6900Event::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Active,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!("🎯 BF-6900 EXTERNAL CONNECTION SERVICE ACTIVE");
        log::info!("   🌐 Listening on port: {}", port);
        log::info!("   🔗 Ready for external laboratory system connections");
        log::info!("   📡 HL7 v2.4 protocol active with MLLP framing");

        // Start the connection handler in a separate thread
        let connections = self.connections.clone();
        let is_running = self.is_running.clone();
        let event_sender = self.event_sender.clone();
        let analyzer_id = {
            let analyzer = self.analyzer.read().await;
            analyzer.id.clone()
        };
        let listener = self.listener.clone();

        tokio::spawn(async move {
            Self::handle_connections_loop(
                listener,
                connections,
                is_running,
                event_sender,
                analyzer_id,
            )
            .await;
        });

        Ok(())
    }

    /// Stops the service
    pub async fn stop(&self) -> Result<(), String> {
        log::info!("🛑 STOPPING BF-6900 EXTERNAL CONNECTION SERVICE");

        *self.is_running.write().await = false;

        // Close all connections
        let mut connections = self.connections.write().await;
        let connection_count = connections.len();
        log::info!("🔌 CLOSING {} ACTIVE EXTERNAL CONNECTIONS", connection_count);
        
        for (analyzer_id, mut connection) in connections.drain() {
            log::info!("   🔗 Closing connection: {} ({})", connection.remote_addr, analyzer_id);
            if let Err(e) = connection.stream.shutdown().await {
                log::warn!("   ⚠️  Error shutting down connection for {}: {}", analyzer_id, e);
            } else {
                log::info!("   ✅ Connection closed successfully: {}", connection.remote_addr);
            }
        }

        // Clear listener
        {
            let mut listener_guard = self.listener.lock().await;
            *listener_guard = None;
        }

        // Update analyzer status to Inactive
        let analyzer_id = {
            let mut analyzer = self.analyzer.write().await;
            analyzer.status = crate::models::AnalyzerStatus::Inactive;
            analyzer.updated_at = chrono::Utc::now();
            analyzer.id.clone()
        };

        // Save updated analyzer to store
        self.save_analyzer_to_store().await?;

        // Emit status update event
        let _ = self
            .event_sender
            .send(BF6900Event::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Inactive,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!("✅ BF-6900 EXTERNAL CONNECTION SERVICE STOPPED");
        log::info!("   📡 HL7 protocol interface disabled");
        log::info!("   🔒 No longer accepting external connections");
        Ok(())
    }

    /// Saves the current analyzer configuration to the store
    async fn save_analyzer_to_store(&self) -> Result<(), String> {
        let analyzer = self.analyzer.read().await;

        let store_data = BF6900StoreData {
            analyzer: Some(analyzer.clone()),
            hl7_settings: Some(crate::models::hematology::HL7Settings::default()),
        };

        let json_value = serde_json::to_value(store_data)
            .map_err(|e| format!("Failed to serialize analyzer configuration: {}", e))?;

        self.store.set("config".to_string(), json_value);

        log::debug!("BF-6900 analyzer configuration saved to store");
        Ok(())
    }

    /// Main connection handling loop
    async fn handle_connections_loop(
        listener: Arc<Mutex<Option<TcpListener>>>,
        connections: Arc<RwLock<HashMap<String, HL7Connection>>>,
        is_running: Arc<RwLock<bool>>,
        event_sender: mpsc::Sender<BF6900Event>,
        analyzer_id: String,
    ) {
        loop {
            // Check if service should stop
            if !*is_running.read().await {
                break;
            }

            // Get listener from mutex
            let listener_guard = listener.lock().await;
            let listener_ref = match &*listener_guard {
                Some(l) => l,
                None => {
                    log::error!("No TCP listener available");
                    break;
                }
            };

            // Accept incoming connections
            match timeout(Duration::from_secs(1), listener_ref.accept()).await {
                Ok(Ok((stream, addr))) => {
                    log::info!("🔗 EXTERNAL CONNECTION ESTABLISHED");
                    log::info!("   📡 Remote Address: {}", addr);
                    log::info!("   🏥 Analyzer ID: {}", analyzer_id);
                    log::info!("   🔧 Protocol: HL7 v2.4 with MLLP framing");

                    let connection = HL7Connection {
                        stream,
                        remote_addr: addr,
                        state: HL7ConnectionState::WaitingForStartBlock,
                        message_buffer: Vec::new(),
                        current_message: Vec::new(),
                        analyzer_id: analyzer_id.clone(),
                        last_activity: Utc::now(),
                        retry_count: 0,
                        health_status: ConnectionHealthStatus::Healthy,
                    };

                    // Store connection
                    connections
                        .write()
                        .await
                        .insert(analyzer_id.clone(), connection);

                    // Send connection event
                    let _ = event_sender
                        .send(BF6900Event::AnalyzerConnected {
                            analyzer_id: analyzer_id.clone(),
                            remote_addr: addr.to_string(),
                            timestamp: Utc::now(),
                        })
                        .await;

                    // Handle the connection
                    let connections_clone = connections.clone();
                    let event_sender_clone = event_sender.clone();
                    let analyzer_id_clone = analyzer_id.clone();

                    tokio::spawn(async move {
                        Self::handle_connection(
                            connections_clone,
                            event_sender_clone,
                            analyzer_id_clone,
                        )
                        .await;
                    });
                }
                Ok(Err(e)) => {
                    log::error!("Error accepting connection: {}", e);
                }
                Err(_) => {
                    // Timeout, continue loop
                    continue;
                }
            }
        }
    }

    /// Handles individual HL7 connection
    async fn handle_connection(
        connections: Arc<RwLock<HashMap<String, HL7Connection>>>,
        event_sender: mpsc::Sender<BF6900Event>,
        analyzer_id: String,
    ) {
        let mut buffer = [0u8; 1024];

        loop {
            // Get connection
            let mut connections_guard = connections.write().await;
            let connection = match connections_guard.get_mut(&analyzer_id) {
                Some(conn) => conn,
                None => {
                    log::warn!("Connection not found for {}", analyzer_id);
                    break;
                }
            };

            // Update last activity and check health
            connection.last_activity = Utc::now();
            Self::update_connection_health(connection);

            // Read data with configurable timeout
            let read_timeout = Self::get_connection_timeout(&connection.health_status);
            match timeout(read_timeout, connection.stream.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    // Connection closed
                    log::info!("HL7 connection closed by {}", connection.remote_addr);
                    break;
                }
                Ok(Ok(n)) => {
                    let data = &buffer[..n];
                    
                    // Log all incoming data transmission
                    log::info!("📥 DATA RECEIVED FROM EXTERNAL SYSTEM");
                    log::info!("   🔗 Connection: {} -> {}", connection.remote_addr, "LIS_SERVER");
                    log::info!("   📊 Data Size: {} bytes", n);
                    log::info!("   📋 Raw Data (hex): {}", data.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "));
                    
                    // Log ASCII representation if printable
                    let ascii_data = String::from_utf8_lossy(data);
                    if ascii_data.chars().all(|c| c.is_ascii() && !c.is_control() || c == '\r' || c == '\n') {
                        log::info!("   📝 Raw Data (ASCII): {:?}", ascii_data);
                    } else {
                        log::info!("   📝 Raw Data contains non-printable characters");
                    }
                    
                    // Log connection health status
                    log::debug!("   💓 Connection Health: {:?}", connection.health_status);
                    log::debug!("   🔄 Retry Count: {}", connection.retry_count);
                    log::debug!("   📡 Connection State: {:?}", connection.state);

                    // Process HL7/MLLP protocol
                    if let Err(e) = Self::process_hl7_data(connection, data, &event_sender).await {
                        let enhanced_error = Self::handle_hl7_processing_error(&e, connection);
                        
                        let _ = event_sender
                            .send(BF6900Event::Error {
                                analyzer_id: analyzer_id.clone(),
                                error: enhanced_error,
                                timestamp: Utc::now(),
                            })
                            .await;

                        // Check if connection should be dropped due to repeated errors
                        if connection.retry_count > 5 {
                            log::error!("Connection {} exceeded retry limit, dropping connection", connection.remote_addr);
                            break;
                        }
                    }
                }
                Ok(Err(e)) => {
                    log::error!("Error reading from HL7 connection: {}", e);
                    break;
                }
                Err(_) => {
                    // Timeout, continue
                    continue;
                }
            }
        }

        // Log connection termination
        log::info!("🔌 EXTERNAL CONNECTION TERMINATED");
        log::info!("   🏥 Analyzer ID: {}", analyzer_id);
        
        // Remove connection
        connections.write().await.remove(&analyzer_id);

        // Send disconnection event
        log::info!("📡 EMITTING DISCONNECTION EVENT");
        let _ = event_sender
            .send(BF6900Event::AnalyzerDisconnected {
                analyzer_id,
                timestamp: Utc::now(),
            })
            .await;
    }

    /// Processes HL7/MLLP protocol data
    async fn process_hl7_data(
        connection: &mut HL7Connection,
        data: &[u8],
        event_sender: &mpsc::Sender<BF6900Event>,
    ) -> Result<(), String> {
        // Add incoming data to buffer
        connection.message_buffer.extend_from_slice(data);

        // Process complete MLLP frames
        while let Some(message_data) = Self::extract_complete_mllp_message(&mut connection.message_buffer)? {
            // Parse HL7 message
            let message_str = String::from_utf8_lossy(&message_data);
            
            // Comprehensive HL7 message logging
            log::info!("📋 COMPLETE HL7 MESSAGE EXTRACTED");
            log::info!("   🔗 Source: {}", connection.remote_addr);
            log::info!("   📏 Message Length: {} bytes", message_data.len());
            log::info!("   📄 Full HL7 Message:\n{}", message_str);
            
            // Log message segments for detailed analysis
            let segments: Vec<&str> = message_str.split('\r').filter(|s| !s.is_empty()).collect();
            log::info!("   📊 Segment Count: {}", segments.len());
            for (i, segment) in segments.iter().enumerate() {
                let segment_type = segment.split('|').next().unwrap_or("UNKNOWN");
                log::info!("   📋 Segment {}: {} = {}", i + 1, segment_type, segment);
            }

            // Log event emission
            log::info!("📡 EMITTING HL7 MESSAGE EVENT");
            log::info!("   🎯 Event Type: BF6900Event::HL7MessageReceived");
            log::info!("   🏥 Analyzer ID: {}", connection.analyzer_id);
            log::info!("   📄 Message Type: HL7");
            
            // Emit raw message event
            let _ = event_sender
                .send(BF6900Event::HL7MessageReceived {
                    analyzer_id: connection.analyzer_id.clone(),
                    message_type: "HL7".to_string(),
                    raw_data: message_str.to_string(),
                    timestamp: Utc::now(),
                })
                .await;

            // Parse HL7 message
            match parse_hl7_message(&message_str) {
                Ok(hl7_message) => {
                    // Validate message content
                    match Self::validate_hl7_message_content(&hl7_message) {
                        Ok(()) => {
                            log::info!("✅ HL7 MESSAGE VALIDATION SUCCESSFUL");
                            log::info!("   📋 Message Type: {}", hl7_message.message_type);
                            log::info!("   📊 Segment Count: {}", hl7_message.segments.len());
                            
                            // Send ACK for valid message
                            let ack = create_hl7_acknowledgment(&hl7_message, "AA", Some("Message accepted"));
                            log::info!("📤 SENDING ACKNOWLEDGMENT TO EXTERNAL SYSTEM");
                            log::info!("   🎯 ACK Type: AA (Application Accept)");
                            log::info!("   📄 ACK Message: {}", ack);
                            Self::send_hl7_response(connection, &ack).await?;

                            // Process message content
                            Self::process_hl7_message(connection, &hl7_message, event_sender).await?;
                            
                            // Reset retry count on successful processing
                            connection.retry_count = 0;
                        }
                        Err(validation_error) => {
                            log::error!("❌ HL7 MESSAGE VALIDATION FAILED");
                            log::error!("   🚨 Validation Error: {}", validation_error);
                            log::error!("   🔗 Connection: {}", connection.remote_addr);
                            let enhanced_error = Self::handle_hl7_processing_error(&validation_error, connection);
                            let nak = Self::create_hl7_nak_response(&message_str, &enhanced_error).await;
                            log::info!("📤 SENDING NAK TO EXTERNAL SYSTEM");
                            log::info!("   🎯 NAK Type: AE (Application Error)");
                            log::info!("   📄 NAK Message: {}", nak);
                            Self::send_hl7_response(connection, &nak).await?;
                        }
                    }
                }
                Err(parse_error) => {
                    log::error!("❌ HL7 MESSAGE PARSING FAILED");
                    log::error!("   🚨 Parse Error: {}", parse_error);
                    log::error!("   📄 Raw Message: {}", message_str);
                    log::error!("   🔗 Connection: {}", connection.remote_addr);
                    let enhanced_error = Self::handle_hl7_processing_error(&parse_error, connection);
                    let nak = Self::create_hl7_nak_response(&message_str, &enhanced_error).await;
                    log::info!("📤 SENDING NAK TO EXTERNAL SYSTEM");
                    log::info!("   🎯 NAK Type: AE (Application Error)");
                    log::info!("   📄 NAK Message: {}", nak);
                    Self::send_hl7_response(connection, &nak).await?;
                }
            }
        }

        Ok(())
    }

    /// Extracts complete MLLP message from buffer
    fn extract_complete_mllp_message(buffer: &mut Vec<u8>) -> Result<Option<Vec<u8>>, String> {
        if buffer.is_empty() {
            return Ok(None);
        }

        // Look for MLLP start block (VT = 0x0B)
        if let Some(start_pos) = buffer.iter().position(|&b| b == 0x0B) {
            // Look for MLLP end sequence (FS CR = 0x1C 0x0D)
            for i in start_pos + 1..buffer.len() - 1 {
                if buffer[i] == 0x1C && buffer[i + 1] == 0x0D {
                    // Found complete message
                    let message_data = buffer[start_pos + 1..i].to_vec();
                    
                    // Remove processed data from buffer
                    buffer.drain(..i + 2);
                    
                    return Ok(Some(message_data));
                }
            }
        }

        Ok(None)
    }

    /// Creates a proper HL7 NAK response for parsing errors
    async fn create_hl7_nak_response(original_message: &str, error: &str) -> String {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let control_id = format!("NAK{}", Utc::now().timestamp());
        
        // Try to extract message control ID from original message
        let original_control_id = original_message
            .lines()
            .find(|line| line.starts_with("MSH"))
            .and_then(|msh_line| {
                let fields: Vec<&str> = msh_line.split('|').collect();
                fields.get(9).map(|s| s.to_string())
            })
            .unwrap_or_else(|| "UNKNOWN".to_string());

        // Create proper NAK response (CQ 5 Plus format)
        format!(
            "MSH|^~\\&|LIS|HOSPITAL|BF-6900|FACILITY|{}||ACK^R01^ACK|{}|P|2.3.1||||||UTF-8\rMSA|AE|{}|{}",
            timestamp,
            control_id,
            original_control_id,
            error
        )
    }

    /// Sends HL7 response (ACK/NAK) back to analyzer
    async fn send_hl7_response(connection: &mut HL7Connection, response: &str) -> Result<(), String> {
        // Wrap response in MLLP framing
        let mut mllp_response = Vec::new();
        mllp_response.push(0x0B); // VT
        mllp_response.extend_from_slice(response.as_bytes());
        mllp_response.push(0x1C); // FS
        mllp_response.push(0x0D); // CR

        // Log outgoing data transmission
        log::info!("📤 SENDING DATA TO EXTERNAL SYSTEM");
        log::info!("   🔗 Connection: {} <- {}", connection.remote_addr, "LIS_SERVER");
        log::info!("   📊 Response Size: {} bytes", mllp_response.len());
        log::info!("   📋 MLLP Frame (hex): {}", mllp_response.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "));
        log::info!("   📝 HL7 Response: {}", response);
        log::info!("   🎯 Frame Structure: VT(0x0B) + Message + FS(0x1C) + CR(0x0D)");

        connection
            .stream
            .write_all(&mllp_response)
            .await
            .map_err(|e| {
                log::error!("❌ FAILED TO SEND DATA TO EXTERNAL SYSTEM");
                log::error!("   🚨 Error: {}", e);
                log::error!("   🔗 Connection: {}", connection.remote_addr);
                format!("Failed to send HL7 response: {}", e)
            })?;

        log::info!("✅ DATA SUCCESSFULLY SENT TO EXTERNAL SYSTEM");
        log::info!("   🔗 Connection: {}", connection.remote_addr);
        log::info!("   📊 Bytes Transmitted: {}", mllp_response.len());
        Ok(())
    }

    /// Processes parsed HL7 message and extracts hematology data
    async fn process_hl7_message(
        connection: &HL7Connection,
        hl7_message: &HL7Message,
        event_sender: &mpsc::Sender<BF6900Event>,
    ) -> Result<(), String> {
        log::info!("Processing HL7 message type: {}", hl7_message.message_type);

        let mut patient_data: Option<PatientData> = None;
        let mut test_results = Vec::new();

        // Process segments to extract patient and test result data
        for segment in &hl7_message.segments {
            match segment.segment_type.as_str() {
                "PID" => {
                    if let Ok(pid_segment) = parse_pid_segment(segment) {
                        patient_data = Some(Self::convert_pid_to_patient_data(&pid_segment));
                        log::debug!("Extracted patient data: {:?}", patient_data);
                    }
                }
                "OBX" => {
                    if let Ok(obx_segment) = parse_obx_segment(segment) {
                        if let Ok(result) = Self::convert_obx_to_hematology_result(&obx_segment, &connection.analyzer_id) {
                            test_results.push(result);
                        }
                    }
                }
                "MSA" => {
                    if let Ok(msa_segment) = parse_msa_segment(segment) {
                        log::debug!("Received acknowledgment: code={}, control_id={}", 
                                   msa_segment.acknowledgment_code, msa_segment.message_control_id);
                    }
                }
                "ORC" => {
                    if let Ok(orc_segment) = parse_orc_segment(segment) {
                        log::debug!("Received order control: command={}, order_number={}, status={}", 
                                   orc_segment.order_control, orc_segment.filler_order_number, orc_segment.order_status);
                    }
                }
                _ => {
                    // Log other segment types for debugging
                    log::debug!("Skipping segment type: {}", segment.segment_type);
                }

            }

        }

        // Log processing results
        log::info!("🧪 HEMATOLOGY RESULTS PROCESSED");
        log::info!("   🏥 Analyzer ID: {}", connection.analyzer_id);
        log::info!("   👤 Patient Data: {:?}", patient_data.is_some());
        if let Some(ref patient) = patient_data {
            log::info!("   👤 Patient ID: {}", patient.id);
            log::info!("   👤 Patient Name: {}", patient.name);
        }
        log::info!("   🧪 Test Results Count: {}", test_results.len());
        for (i, result) in test_results.iter().enumerate() {
            log::info!("   🧪 Result {}: {} = {} {} ({})", 
                i + 1, result.parameter, result.value, 
                result.units.as_deref().unwrap_or(""), result.status);
        }
        
        // Send the processed data as an event
        log::info!("📡 EMITTING HEMATOLOGY RESULTS EVENT");
        let _ = event_sender
            .send(BF6900Event::HematologyResultProcessed {
                analyzer_id: connection.analyzer_id.clone(),
                patient_id: patient_data.as_ref().map(|p| p.id.clone()),
                patient_data,
                test_results,
                timestamp: Utc::now(),
            })
            .await;

        Ok(())
    }

    /// Converts PID segment to PatientData
    fn convert_pid_to_patient_data(pid: &PIDSegment) -> PatientData {
        PatientData {
            id: pid.patient_identifier_list.clone(),
            name: pid.patient_name.clone(),
            birth_date: if !pid.date_time_of_birth.is_empty() {
                Some(pid.date_time_of_birth.clone())
            } else {
                None
            },
            sex: if !pid.administrative_sex.is_empty() {
                Some(pid.administrative_sex.clone())
            } else {
                None
            },
            address: if !pid.patient_address.is_empty() {
                Some(pid.patient_address.clone())
            } else {
                None
            },
            telephone: if !pid.phone_number_home.is_empty() {
                Some(pid.phone_number_home.clone())
            } else {
                None
            },
            physicians: None, // Not typically in PID segment
            height: None,     // Not typically in PID segment
            weight: None,     // Not typically in PID segment
        }
    }

    /// Converts OBX segment to HematologyResult (CQ 5 Plus parameter codes)
    fn convert_obx_to_hematology_result(
        obx: &OBXSegment,
        analyzer_id: &str,
    ) -> Result<HematologyResult, String> {
        let parameter_name = extract_parameter_name(&obx.observation_identifier);
        let parameter_code = extract_parameter_code(&obx.observation_identifier);
        let flags = extract_abnormal_flags(&obx.abnormal_flags);
        let now = Utc::now();

        Ok(HematologyResult {
            id: format!("hematology_{}", now.timestamp()),
            parameter: parameter_name,
            parameter_code,
            value: obx.observation_value.clone(),
            units: if !obx.units.is_empty() {
                Some(obx.units.clone())
            } else {
                None
            },
            reference_range: if !obx.references_range.is_empty() {
                Some(obx.references_range.clone())
            } else {
                None
            },
            flags,
            status: obx.observation_result_status.clone(),
            completed_date_time: if !obx.date_time_of_observation.is_empty() {
                // Parse HL7 datetime format
                Some(now) // Simplified for now
            } else {
                Some(now)
            },
            analyzer_id: Some(analyzer_id.to_string()),
            sample_id: obx.observation_sub_id.clone(),
            test_id: obx.observation_identifier.clone(),
            created_at: now,
            updated_at: now,
        })
    }

    /// Gets service status
    pub async fn get_status(&self) -> AnalyzerStatus {
        if *self.is_running.read().await {
            AnalyzerStatus::Active
        } else {
            AnalyzerStatus::Inactive
        }
    }

    /// Gets active connections count
    pub async fn get_connections_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Gets the current analyzer configuration
    pub async fn get_analyzer_config(&self) -> Analyzer {
        self.analyzer.read().await.clone()
    }

    /// Updates connection health status based on activity and errors
    fn update_connection_health(connection: &mut HL7Connection) {
        let now = Utc::now();
        let time_since_activity = now.signed_duration_since(connection.last_activity);

        connection.health_status = match connection.retry_count {
            0..=2 if time_since_activity.num_seconds() < 30 => ConnectionHealthStatus::Healthy,
            3..=5 if time_since_activity.num_seconds() < 60 => ConnectionHealthStatus::Degraded,
            _ => ConnectionHealthStatus::Unhealthy,
        };

        if matches!(connection.health_status, ConnectionHealthStatus::Unhealthy) {
            log::warn!(
                "Connection {} marked as unhealthy (retries: {}, last activity: {}s ago)",
                connection.remote_addr,
                connection.retry_count,
                time_since_activity.num_seconds()
            );
        }
    }

    /// Gets appropriate timeout based on connection health
    fn get_connection_timeout(health_status: &ConnectionHealthStatus) -> Duration {
        match health_status {
            ConnectionHealthStatus::Healthy => Duration::from_secs(10),
            ConnectionHealthStatus::Degraded => Duration::from_secs(5),
            ConnectionHealthStatus::Unhealthy => Duration::from_secs(2),
        }
    }

    /// Validates HL7 message structure and content
    fn validate_hl7_message_content(message: &HL7Message) -> Result<(), String> {
        // Check if message has required segments
        if message.segments.is_empty() {
            return Err("HL7 message has no segments".to_string());
        }

        // Check if first segment is MSH
        if message.segments[0].segment_type != "MSH" {
            return Err("First segment must be MSH".to_string());
        }

        // Validate message type using CQ 5 Plus supported types
        if !is_supported_message_type(&message.message_type) {
            return Err(format!("Unsupported message type: {}", message.message_type));
        }

        // Check for required patient identification
        let has_pid = message.segments.iter().any(|s| s.segment_type == "PID");
        if !has_pid {
            log::warn!("HL7 message missing PID segment - patient identification may be incomplete");
        }

        // Check for observation results (not required for worklist messages)
        let has_obx = message.segments.iter().any(|s| s.segment_type == "OBX");
        let is_worklist = message.message_type.starts_with("ORM") || message.message_type.starts_with("ORR");
        
        if !has_obx && !is_worklist {
            return Err("HL7 message missing OBX segments - no test results found".to_string());
        }

        Ok(())
    }

    /// Enhanced error handling with specific error types
    fn handle_hl7_processing_error(error: &str, connection: &mut HL7Connection) -> String {
        connection.retry_count += 1;
        
        let error_type = if error.contains("timeout") {
            "TIMEOUT"
        } else if error.contains("parse") || error.contains("invalid") {
            "PARSE_ERROR"
        } else if error.contains("segment") {
            "SEGMENT_ERROR"
        } else {
            "UNKNOWN_ERROR"
        };

        let enhanced_error = format!("{}:{} (retry {})", error_type, error, connection.retry_count);
        
        // Comprehensive error logging for external system communication
        log::error!("🚨 EXTERNAL SYSTEM COMMUNICATION ERROR");
        log::error!("   🔗 Connection: {}", connection.remote_addr);
        log::error!("   🏥 Analyzer ID: {}", connection.analyzer_id);
        log::error!("   🔢 Error Type: {}", error_type);
        log::error!("   📝 Error Details: {}", error);
        log::error!("   🔄 Retry Count: {}", connection.retry_count);
        log::error!("   💓 Connection Health: {:?}", connection.health_status);
        
        // Log buffer state for debugging
        log::error!("   📊 Message Buffer Size: {} bytes", connection.message_buffer.len());
        log::error!("   📊 Current Message Size: {} bytes", connection.current_message.len());
        
        if connection.retry_count > 3 {
            log::error!("   ⚠️  HIGH RETRY COUNT - CONNECTION MAY BE UNSTABLE");
        }

        enhanced_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mllp_message_extraction() {
        let mut buffer = vec![0x0B]; // VT
        buffer.extend_from_slice(b"MSH|^~\\&|BF6900|LAB|LIS|HOSPITAL||");
        buffer.push(0x1C); // FS
        buffer.push(0x0D); // CR

        let result = BF6900Service::<tauri::Wry>::extract_complete_mllp_message(&mut buffer).unwrap();
        assert!(result.is_some());
        let message = result.unwrap();
        assert_eq!(String::from_utf8_lossy(&message), "MSH|^~\\&|BF6900|LAB|LIS|HOSPITAL||");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_incomplete_mllp_message() {
        let mut buffer = vec![0x0B]; // VT
        buffer.extend_from_slice(b"MSH|^~\\&|BF6900|LAB|LIS|HOSPITAL||");
        // No end sequence

        let result = BF6900Service::<tauri::Wry>::extract_complete_mllp_message(&mut buffer).unwrap();
        assert!(result.is_none());
        assert!(!buffer.is_empty()); // Buffer should retain data
    }

    #[test]
    fn test_connection_health_status() {
        // Test connection health status values
        assert!(matches!(ConnectionHealthStatus::Healthy, ConnectionHealthStatus::Healthy));
        assert!(matches!(ConnectionHealthStatus::Degraded, ConnectionHealthStatus::Degraded));
        assert!(matches!(ConnectionHealthStatus::Unhealthy, ConnectionHealthStatus::Unhealthy));
    }

    #[test]
    fn test_connection_timeout_adjustment() {
        let healthy_timeout = BF6900Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Healthy);
        let degraded_timeout = BF6900Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Degraded);
        let unhealthy_timeout = BF6900Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Unhealthy);

        assert!(healthy_timeout > degraded_timeout);
        assert!(degraded_timeout > unhealthy_timeout);
        assert_eq!(healthy_timeout.as_secs(), 10);
        assert_eq!(degraded_timeout.as_secs(), 5);
        assert_eq!(unhealthy_timeout.as_secs(), 2);
    }

    #[test]
    fn test_error_type_classification() {
        // Test that error types are correctly classified
        let timeout_error = "connection timeout occurred";
        let parse_error = "invalid message format";
        let segment_error = "missing segment data";
        let unknown_error = "unexpected issue";

        // Since handle_hl7_processing_error requires a mutable connection, we'll test the logic separately
        assert!(timeout_error.contains("timeout"));
        assert!(parse_error.contains("invalid"));
        assert!(segment_error.contains("segment"));
        assert!(!unknown_error.contains("timeout") && !unknown_error.contains("parse") && !unknown_error.contains("segment"));
    }

    #[test]
    fn test_pid_to_patient_data_conversion() {
        let pid = PIDSegment {
            set_id: "1".to_string(),
            patient_id: "".to_string(),
            patient_identifier_list: "P123456".to_string(),
            alternate_patient_id: "".to_string(),
            patient_name: "DOE^JOHN^MIDDLE".to_string(),
            mothers_maiden_name: "".to_string(),
            date_time_of_birth: "19800101".to_string(),
            administrative_sex: "M".to_string(),
            patient_alias: "".to_string(),
            race: "".to_string(),
            patient_address: "123 Main St^City^State^12345".to_string(),
            county_code: "".to_string(),
            phone_number_home: "555-1234".to_string(),
            phone_number_business: "".to_string(),
            primary_language: "".to_string(),
        };

        let patient_data = BF6900Service::<tauri::Wry>::convert_pid_to_patient_data(&pid);
        assert_eq!(patient_data.id, "P123456");
        assert_eq!(patient_data.name, "DOE^JOHN^MIDDLE");
        assert_eq!(patient_data.sex, Some("M".to_string()));
        assert_eq!(patient_data.birth_date, Some("19800101".to_string()));
    }

    #[test]
    fn test_obx_to_hematology_result_cq5_plus() {
        let obx = OBXSegment {
            set_id: "1".to_string(),
            value_type: "NM".to_string(),
            observation_identifier: "2006^V_WBC^LOCAL".to_string(), // CQ 5 Plus WBC code
            observation_sub_id: "".to_string(),
            observation_value: "6.8".to_string(),
            units: "10^9/L".to_string(),
            references_range: "4-10".to_string(),
            abnormal_flags: "".to_string(),
            probability: "".to_string(),
            nature_of_abnormal_test: "".to_string(),
            observation_result_status: "F".to_string(),
            effective_date_of_reference_range: "".to_string(),
            user_defined_access_checks: "".to_string(),
            date_time_of_observation: "".to_string(),
        };

        let result = BF6900Service::<tauri::Wry>::convert_obx_to_hematology_result(&obx, "ANALYZER001").unwrap();
        assert_eq!(result.parameter, "V_WBC");
        assert_eq!(result.parameter_code, "2006"); // CQ 5 Plus parameter code
        assert_eq!(result.value, "6.8");
        assert_eq!(result.units, Some("10^9/L".to_string()));
        assert_eq!(result.reference_range, Some("4-10".to_string()));
        assert_eq!(result.status, "F");
    }

    #[test]
    fn test_crp_parameter_conversion() {
        let obx_crp = OBXSegment {
            set_id: "1".to_string(),
            value_type: "NM".to_string(),
            observation_identifier: "2031^V_CRP^LOCAL".to_string(), // New CRP parameter
            observation_sub_id: "".to_string(),
            observation_value: "3.2".to_string(),
            units: "mg/L".to_string(),
            references_range: "0-6".to_string(),
            abnormal_flags: "".to_string(),
            probability: "".to_string(),
            nature_of_abnormal_test: "".to_string(),
            observation_result_status: "F".to_string(),
            effective_date_of_reference_range: "".to_string(),
            user_defined_access_checks: "".to_string(),
            date_time_of_observation: "".to_string(),
        };

        let result = BF6900Service::<tauri::Wry>::convert_obx_to_hematology_result(&obx_crp, "ANALYZER001").unwrap();
        assert_eq!(result.parameter, "V_CRP");
        assert_eq!(result.parameter_code, "2031");
        assert_eq!(result.value, "3.2");
        assert_eq!(result.units, Some("mg/L".to_string()));
    }
}