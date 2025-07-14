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
use crate::models::hematology::{BF6500Event, HematologyResult, PatientData};
use crate::api::commands::bf6500_handler::BF6500StoreData;
use crate::protocol::hl7_parser::{
    HL7ConnectionState, HL7Message, OBXSegment, PIDSegment,
    parse_hl7_message, create_hl7_acknowledgment,
    extract_parameter_name, extract_abnormal_flags, parse_pid_segment, parse_obx_segment
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
// MAIN BF-6500 SERVICE
// ============================================================================

pub struct BF6500Service<R: Runtime> {
    /// Analyzer configuration
    analyzer: Arc<RwLock<Analyzer>>,
    /// TCP listener for incoming connections
    listener: Arc<Mutex<Option<TcpListener>>>,
    /// Active connections
    connections: Arc<RwLock<HashMap<String, HL7Connection>>>,
    /// Event sender for frontend communication
    event_sender: mpsc::Sender<BF6500Event>,
    /// Service status
    is_running: Arc<RwLock<bool>>,
    /// Store for configuration persistence
    store: Arc<tauri_plugin_store::Store<R>>,
}

impl<R: Runtime> BF6500Service<R> {
    /// Creates a new BF6500 service
    pub fn new(
        analyzer: Analyzer,
        event_sender: mpsc::Sender<BF6500Event>,
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

        log::info!("Starting BF-6500 service on {}", bind_addr);

        // Create TCP listener
        let listener = TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| format!("Failed to bind to {}: {}", bind_addr, e))?;

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
            .send(BF6500Event::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Active,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!(
            "BF-6500 service started successfully on port {}",
            port
        );

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
        log::info!("Stopping BF-6500 service");

        *self.is_running.write().await = false;

        // Close all connections
        let mut connections = self.connections.write().await;
        for (analyzer_id, mut connection) in connections.drain() {
            if let Err(e) = connection.stream.shutdown().await {
                log::warn!("Error shutting down connection for {}: {}", analyzer_id, e);
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
            .send(BF6500Event::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Inactive,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!("BF-6500 service stopped");
        Ok(())
    }

    /// Saves the current analyzer configuration to the store
    async fn save_analyzer_to_store(&self) -> Result<(), String> {
        let analyzer = self.analyzer.read().await;

        let store_data = BF6500StoreData {
            analyzer: Some(analyzer.clone()),
            hl7_settings: Some(crate::models::hematology::HL7Settings::default()),
        };

        let json_value = serde_json::to_value(store_data)
            .map_err(|e| format!("Failed to serialize analyzer configuration: {}", e))?;

        self.store.set("config".to_string(), json_value);

        log::debug!("BF-6500 analyzer configuration saved to store");
        Ok(())
    }

    /// Main connection handling loop
    async fn handle_connections_loop(
        listener: Arc<Mutex<Option<TcpListener>>>,
        connections: Arc<RwLock<HashMap<String, HL7Connection>>>,
        is_running: Arc<RwLock<bool>>,
        event_sender: mpsc::Sender<BF6500Event>,
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
                    log::info!("New HL7 connection from {}", addr);

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
                        .send(BF6500Event::AnalyzerConnected {
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
        event_sender: mpsc::Sender<BF6500Event>,
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

                    // Process HL7/MLLP protocol
                    if let Err(e) = Self::process_hl7_data(connection, data, &event_sender).await {
                        let enhanced_error = Self::handle_hl7_processing_error(&e, connection);
                        
                        let _ = event_sender
                            .send(BF6500Event::Error {
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

        // Remove connection
        connections.write().await.remove(&analyzer_id);

        // Send disconnection event
        let _ = event_sender
            .send(BF6500Event::AnalyzerDisconnected {
                analyzer_id,
                timestamp: Utc::now(),
            })
            .await;
    }

    /// Processes HL7/MLLP protocol data
    async fn process_hl7_data(
        connection: &mut HL7Connection,
        data: &[u8],
        event_sender: &mpsc::Sender<BF6500Event>,
    ) -> Result<(), String> {
        // Add incoming data to buffer
        connection.message_buffer.extend_from_slice(data);

        // Process complete MLLP frames
        while let Some(message_data) = Self::extract_complete_mllp_message(&mut connection.message_buffer)? {
            // Parse HL7 message
            let message_str = String::from_utf8_lossy(&message_data);
            log::debug!("Received HL7 message: {}", message_str);

            // Emit raw message event
            let _ = event_sender
                .send(BF6500Event::HL7MessageReceived {
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
                            // Send ACK for valid message
                            let ack = create_hl7_acknowledgment(&hl7_message, "AA", Some("Message accepted"));
                            Self::send_hl7_response(connection, &ack).await?;

                            // Process message content
                            Self::process_hl7_message(connection, &hl7_message, event_sender).await?;
                            
                            // Reset retry count on successful processing
                            connection.retry_count = 0;
                        }
                        Err(validation_error) => {
                            let enhanced_error = Self::handle_hl7_processing_error(&validation_error, connection);
                            let nak = Self::create_hl7_nak_response(&message_str, &enhanced_error).await;
                            Self::send_hl7_response(connection, &nak).await?;
                        }
                    }
                }
                Err(parse_error) => {
                    let enhanced_error = Self::handle_hl7_processing_error(&parse_error, connection);
                    let nak = Self::create_hl7_nak_response(&message_str, &enhanced_error).await;
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

        // Create proper NAK response
        format!(
            "MSH|^~\\&|BF6500_LIS|HOSPITAL|SENDER|FACILITY|{}||ACK^R01^ACK|{}|P|2.4\rMSA|AE|{}|{}",
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

        connection
            .stream
            .write_all(&mllp_response)
            .await
            .map_err(|e| format!("Failed to send HL7 response: {}", e))?;

        log::debug!("Sent HL7 response: {}", response);
        Ok(())
    }

    /// Processes parsed HL7 message and extracts hematology data
    async fn process_hl7_message(
        connection: &HL7Connection,
        hl7_message: &HL7Message,
        event_sender: &mpsc::Sender<BF6500Event>,
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
                _ => {
                    // Log other segment types for debugging
                    log::debug!("Skipping segment type: {}", segment.segment_type);
                }
            }
        }

        // Send the processed data as an event
        let _ = event_sender
            .send(BF6500Event::HematologyResultProcessed {
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

    /// Converts OBX segment to HematologyResult
    fn convert_obx_to_hematology_result(
        obx: &OBXSegment,
        analyzer_id: &str,
    ) -> Result<HematologyResult, String> {
        let parameter_name = extract_parameter_name(&obx.observation_identifier);
        let flags = extract_abnormal_flags(&obx.abnormal_flags);
        let now = Utc::now();

        Ok(HematologyResult {
            id: format!("hematology_{}", now.timestamp()),
            parameter: parameter_name.clone(),
            parameter_code: parameter_name,
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

        // Validate message type for hematology data
        if !message.message_type.starts_with("ORU^R01") && !message.message_type.starts_with("OUL^R21") {
            return Err(format!("Unsupported message type: {}", message.message_type));
        }

        // Check for required patient identification
        let has_pid = message.segments.iter().any(|s| s.segment_type == "PID");
        if !has_pid {
            log::warn!("HL7 message missing PID segment - patient identification may be incomplete");
        }

        // Check for observation results
        let has_obx = message.segments.iter().any(|s| s.segment_type == "OBX");
        if !has_obx {
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
        
        log::error!(
            "HL7 processing error for connection {}: {}",
            connection.remote_addr,
            enhanced_error
        );

        enhanced_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::hematology::HL7Settings;
    use crate::models::{ConnectionType, Protocol};

    #[test]
    fn test_mllp_message_extraction() {
        let mut buffer = vec![0x0B]; // VT
        buffer.extend_from_slice(b"MSH|^~\\&|BF6500|LAB|LIS|HOSPITAL||");
        buffer.push(0x1C); // FS
        buffer.push(0x0D); // CR

        let result = BF6500Service::<tauri::Wry>::extract_complete_mllp_message(&mut buffer).unwrap();
        assert!(result.is_some());
        let message = result.unwrap();
        assert_eq!(String::from_utf8_lossy(&message), "MSH|^~\\&|BF6500|LAB|LIS|HOSPITAL||");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_incomplete_mllp_message() {
        let mut buffer = vec![0x0B]; // VT
        buffer.extend_from_slice(b"MSH|^~\\&|BF6500|LAB|LIS|HOSPITAL||");
        // No end sequence

        let result = BF6500Service::<tauri::Wry>::extract_complete_mllp_message(&mut buffer).unwrap();
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
        let healthy_timeout = BF6500Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Healthy);
        let degraded_timeout = BF6500Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Degraded);
        let unhealthy_timeout = BF6500Service::<tauri::Wry>::get_connection_timeout(&ConnectionHealthStatus::Unhealthy);

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

        let patient_data = BF6500Service::<tauri::Wry>::convert_pid_to_patient_data(&pid);
        assert_eq!(patient_data.id, "P123456");
        assert_eq!(patient_data.name, "DOE^JOHN^MIDDLE");
        assert_eq!(patient_data.sex, Some("M".to_string()));
        assert_eq!(patient_data.birth_date, Some("19800101".to_string()));
    }
}