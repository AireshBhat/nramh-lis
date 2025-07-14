use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::Runtime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::timeout;

use crate::models::{Analyzer, AnalyzerStatus};

// ============================================================================
// EVENT TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MerilEvent {
    /// Analyzer connected
    AnalyzerConnected {
        analyzer_id: String,
        remote_addr: String,
        timestamp: DateTime<Utc>,
    },
    /// Analyzer disconnected
    AnalyzerDisconnected {
        analyzer_id: String,
        timestamp: DateTime<Utc>,
    },
    /// ASTM message received
    AstmMessageReceived {
        analyzer_id: String,
        message_type: String,
        raw_data: String,
        timestamp: DateTime<Utc>,
    },
    /// Lab result processed
    LabResultProcessed {
        analyzer_id: String,
        patient_id: Option<String>,
        patient_data: Option<PatientData>,
        test_results: Vec<TestResult>,
        timestamp: DateTime<Utc>,
    },
    /// Analyzer status updated
    AnalyzerStatusUpdated {
        analyzer_id: String,
        status: crate::models::AnalyzerStatus,
        timestamp: DateTime<Utc>,
    },
    /// Error occurred
    Error {
        analyzer_id: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: String,
    pub test_id: String,
    pub sample_id: String,
    pub value: String,
    pub units: Option<String>,
    pub reference_range: Option<String>,
    pub flags: Vec<String>,
    pub status: String,
    pub completed_date_time: Option<DateTime<Utc>>,
    pub analyzer_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientData {
    pub id: String,
    pub name: String,
    pub birth_date: Option<String>,
    pub sex: Option<String>,
    pub address: Option<String>,
    pub telephone: Option<String>,
    pub physicians: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
}

// ============================================================================
// ASTM PROTOCOL CONSTANTS
// ============================================================================

const ASTM_ENQ: u8 = 0x05; // ENQ - Enquiry
const ASTM_ACK: u8 = 0x06; // ACK - Acknowledgment
const ASTM_NAK: u8 = 0x15; // NAK - Negative Acknowledgment
const ASTM_EOT: u8 = 0x04; // EOT - End of Transmission
const ASTM_STX: u8 = 0x02; // STX - Start of Text
const ASTM_ETX: u8 = 0x03; // ETX - End of Text
const ASTM_ETB: u8 = 0x17; // ETB - End of Transmission Block
const ASTM_CR: u8 = 0x0D; // CR - Carriage Return
const ASTM_LF: u8 = 0x0A; // LF - Line Feed

// ============================================================================
// CONNECTION STATE
// ============================================================================

#[derive(Debug, Clone)]
pub enum ConnectionState {
    WaitingForEnq,
    WaitingForFrame,
    ProcessingFrame,
    WaitingForChecksum,
    WaitingForCR,
    WaitingForLF,
    Complete,
}

#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub remote_addr: SocketAddr,
    pub state: ConnectionState,
    pub frame_buffer: Vec<Vec<u8>>, // Store multiple frames
    pub current_frame: Vec<u8>,     // Current frame being built
    pub analyzer_id: String,
}

// ============================================================================
// MAIN SERVICE
// ============================================================================

pub struct AutoQuantMerilService<R: Runtime> {
    /// Analyzer configuration
    analyzer: Arc<RwLock<Analyzer>>,
    /// TCP listener for incoming connections
    listener: Arc<Mutex<Option<TcpListener>>>,
    /// Active connections
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    /// Event sender for frontend communication
    event_sender: mpsc::Sender<MerilEvent>,
    /// Service status
    is_running: Arc<RwLock<bool>>,
    /// Store for configuration persistence
    store: Arc<tauri_plugin_store::Store<R>>,
}

impl<R: Runtime> AutoQuantMerilService<R> {
    /// Creates a new AutoQuantMeril service
    pub fn new(
        analyzer: Analyzer,
        event_sender: mpsc::Sender<MerilEvent>,
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

        log::info!("Starting AutoQuantMeril service on {}", bind_addr);

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
            .send(MerilEvent::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Active,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!(
            "AutoQuantMeril service started successfully on port {}",
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
        log::info!("Stopping AutoQuantMeril service");

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
            .send(MerilEvent::AnalyzerStatusUpdated {
                analyzer_id: analyzer_id.clone(),
                status: crate::models::AnalyzerStatus::Inactive,
                timestamp: chrono::Utc::now(),
            })
            .await;

        log::info!("AutoQuantMeril service stopped");
        Ok(())
    }

    /// Saves the current analyzer configuration to the store
    async fn save_analyzer_to_store(&self) -> Result<(), String> {
        let analyzer = self.analyzer.read().await;

        let store_data = crate::api::commands::meril_handler::MerilStoreData {
            analyzer: Some(analyzer.clone()),
        };

        let json_value = serde_json::to_value(store_data)
            .map_err(|e| format!("Failed to serialize analyzer configuration: {}", e))?;

        self.store.set("config".to_string(), json_value);

        log::debug!("Analyzer configuration saved to store");
        Ok(())
    }

    /// Main connection handling loop
    async fn handle_connections_loop(
        listener: Arc<Mutex<Option<TcpListener>>>,
        connections: Arc<RwLock<HashMap<String, Connection>>>,
        is_running: Arc<RwLock<bool>>,
        event_sender: mpsc::Sender<MerilEvent>,
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
                    log::info!("New connection from {}", addr);

                    let connection = Connection {
                        stream,
                        remote_addr: addr,
                        state: ConnectionState::WaitingForEnq,
                        frame_buffer: Vec::new(),
                        current_frame: Vec::new(),
                        analyzer_id: analyzer_id.clone(),
                    };

                    // Store connection
                    connections
                        .write()
                        .await
                        .insert(analyzer_id.clone(), connection);

                    // Send connection event
                    let _ = event_sender
                        .send(MerilEvent::AnalyzerConnected {
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

    /// Handles individual connection
    async fn handle_connection(
        connections: Arc<RwLock<HashMap<String, Connection>>>,
        event_sender: mpsc::Sender<MerilEvent>,
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

            // Read data
            match timeout(Duration::from_secs(5), connection.stream.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    // Connection closed
                    log::info!("Connection closed by {}", connection.remote_addr);
                    break;
                }
                Ok(Ok(n)) => {
                    let data = &buffer[..n];

                    // Process ASTM protocol
                    if let Err(e) = Self::process_astm_data(connection, data, &event_sender).await {
                        log::error!("Error processing ASTM data: {}", e);

                        let _ = event_sender
                            .send(MerilEvent::Error {
                                analyzer_id: analyzer_id.clone(),
                                error: e,
                                timestamp: Utc::now(),
                            })
                            .await;
                    }
                }
                Ok(Err(e)) => {
                    log::error!("Error reading from connection: {}", e);
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
            .send(MerilEvent::AnalyzerDisconnected {
                analyzer_id,
                timestamp: Utc::now(),
            })
            .await;
    }

    /// Processes ASTM protocol data
    async fn process_astm_data(
        connection: &mut Connection,
        data: &[u8],
        event_sender: &mpsc::Sender<MerilEvent>,
    ) -> Result<(), String> {
        for &byte in data {
            match connection.state {
                ConnectionState::WaitingForEnq => {
                    if byte == ASTM_ENQ {
                        // Send ACK
                        connection
                            .stream
                            .write_all(&[ASTM_ACK])
                            .await
                            .map_err(|e| format!("Failed to send ACK: {}", e))?;

                        connection.state = ConnectionState::WaitingForFrame;
                        log::debug!("Received ENQ, sent ACK, waiting for frame");
                    }
                }
                ConnectionState::WaitingForFrame => {
                    if byte == ASTM_STX {
                        connection.current_frame.clear();
                        connection.current_frame.push(byte);
                        connection.state = ConnectionState::ProcessingFrame;
                        log::debug!("Received STX, processing frame");
                    } else if byte == ASTM_EOT {
                        // End of transmission
                        log::info!("Received EOT, transmission complete");

                        // Process complete message
                        Self::process_complete_message(connection, event_sender).await?;

                        // Send ACK for EOT
                        connection
                            .stream
                            .write_all(&[ASTM_ACK])
                            .await
                            .map_err(|e| format!("Failed to send ACK for EOT: {}", e))?;

                        // Clear frame buffer for next transmission
                        connection.frame_buffer.clear();
                        connection.current_frame.clear();

                        // Reset state for next transmission
                        connection.state = ConnectionState::WaitingForEnq;
                        log::info!("Transmission complete, ready for next transmission");

                        // Break out of the loop - transmission is complete
                        // The connection will be ready for the next transmission when it receives ENQ again
                        break;
                    } else {
                        log::debug!(
                            "Unexpected byte in WaitingForFrame: 0x{:02X} ('{}')",
                            byte,
                            byte as char
                        );
                    }
                }
                ConnectionState::ProcessingFrame => {
                    connection.current_frame.push(byte);

                    if byte == ASTM_ETX || byte == ASTM_ETB {
                        log::debug!("Received ETX or ETB, waiting for checksum");
                        connection.state = ConnectionState::WaitingForChecksum;
                    }
                }
                ConnectionState::WaitingForChecksum => {
                    // Store checksum byte
                    connection.current_frame.push(byte);
                    log::debug!("Received checksum: 0x{:02X}, waiting for CR", byte);
                    connection.state = ConnectionState::WaitingForCR;
                }
                ConnectionState::WaitingForCR => {
                    if byte == ASTM_CR {
                        connection.current_frame.push(byte);
                        log::debug!("Received CR, waiting for LF");
                        connection.state = ConnectionState::WaitingForLF;
                    } else {
                        log::error!("Expected CR (0x0D), got 0x{:02X}", byte);
                        return Err("Invalid frame format: expected CR".to_string());
                    }
                }
                ConnectionState::WaitingForLF => {
                    if byte == ASTM_LF {
                        connection.current_frame.push(byte);
                        log::debug!("Received LF, processing complete frame");

                        // Now process the complete frame
                        if let Err(e) = Self::process_frame(connection, event_sender).await {
                            // Send NAK on error
                            connection
                                .stream
                                .write_all(&[ASTM_NAK])
                                .await
                                .map_err(|e| format!("Failed to send NAK: {}", e))?;
                            return Err(e);
                        }

                        // Send ACK
                        connection
                            .stream
                            .write_all(&[ASTM_ACK])
                            .await
                            .map_err(|e| format!("Failed to send ACK: {}", e))?;

                        connection.current_frame.clear();
                        connection.state = ConnectionState::WaitingForFrame;
                    } else {
                        log::error!("Expected LF (0x0A), got 0x{:02X}", byte);
                        return Err("Invalid frame format: expected LF".to_string());
                    }
                }
                ConnectionState::Complete => {
                    // Should not reach here - transmission is complete
                    log::warn!(
                        "Unexpected data after EOT in Complete state: 0x{:02X}",
                        byte
                    );
                    // Break out of the loop as transmission is complete
                    break;
                }
            }
        }

        Ok(())
    }

    /// Processes a single ASTM frame
    async fn process_frame(
        connection: &mut Connection,
        event_sender: &mpsc::Sender<MerilEvent>,
    ) -> Result<(), String> {
        // Debug: Log the raw frame
        log::debug!("Processing frame: {:?}", connection.current_frame);

        // Log frame structure for debugging
        if connection.current_frame.len() >= 6 {
            let frame_number = connection.current_frame[0];
            let stx = connection.current_frame[1];
            let etx_pos = connection.current_frame.len() - 4;
            let etx = connection.current_frame[etx_pos];
            let checksum = connection.current_frame[connection.current_frame.len() - 3];
            let cr = connection.current_frame[connection.current_frame.len() - 2];
            let lf = connection.current_frame[connection.current_frame.len() - 1];

            log::debug!(
                "Frame structure: FN=0x{:02X}, STX=0x{:02X}, ETX=0x{:02X}, CS=0x{:02X}, CR=0x{:02X}, LF=0x{:02X}",
                frame_number, stx, etx, checksum, cr, lf
            );
        }

        // Validate checksum
        if !Self::validate_checksum(&connection.current_frame) {
            log::error!(
                "Checksum validation failed for frame: {:?}",
                connection.current_frame
            );
        }

        // Extract frame data (remove frame number, STX, ETX, checksum, CR, LF)
        let frame_data = Self::extract_frame_data(&connection.current_frame)?;

        // Parse ASTM record
        let record_type = Self::parse_record_type(&frame_data)?;

        log::debug!(
            "Processed ASTM frame: {} - {}",
            record_type,
            String::from_utf8_lossy(&frame_data)
        );

        // Store the completed frame for later processing
        connection
            .frame_buffer
            .push(connection.current_frame.clone());

        // Send event
        let _ = event_sender
            .send(MerilEvent::AstmMessageReceived {
                analyzer_id: connection.analyzer_id.clone(),
                message_type: record_type,
                raw_data: String::from_utf8_lossy(&frame_data).to_string(),
                timestamp: Utc::now(),
            })
            .await;

        Ok(())
    }

    /// Processes complete ASTM message
    async fn process_complete_message(
        connection: &mut Connection,
        event_sender: &mpsc::Sender<MerilEvent>,
    ) -> Result<(), String> {
        log::info!(
            "Processing complete ASTM message from {}",
            connection.remote_addr
        );

        // Parse all collected frames to extract patient and test result data
        let mut patient_data: Option<PatientData> = None;
        let mut test_results = Vec::new();

        // Process each frame to extract patient and result data
        for frame in &connection.frame_buffer {
            if let Ok(frame_data) = Self::extract_frame_data(frame) {
                let record_type = Self::parse_record_type(&frame_data)?;

                match record_type.as_str() {
                    "Patient" => {
                        if let Ok(patient) = Self::parse_patient_record(&frame_data) {
                            log::debug!("Patient data: {:?}", patient);
                            patient_data = Some(patient);
                        }
                    }
                    "Result" => {
                        if let Ok(mut result) = Self::parse_result_record(&frame_data) {
                            result.analyzer_id = Some(connection.analyzer_id.clone());
                            test_results.push(result);
                        }
                    }
                    _ => {
                        // Log other record types for debugging
                        log::debug!("Skipping record type: {}", record_type);
                    }
                }
            }
        }

        // Send the processed data as an event
        let _ = event_sender
            .send(MerilEvent::LabResultProcessed {
                analyzer_id: connection.analyzer_id.clone(),
                patient_id: patient_data.as_ref().map(|p| p.id.clone()),
                patient_data,
                test_results,
                timestamp: Utc::now(),
            })
            .await;

        Ok(())
    }

    /// Validates ASTM frame checksum
    fn validate_checksum(frame: &[u8]) -> bool {
        if frame.len() < 6 {
            return false;
        }

        // ASTM frame format: FrameNumber + STX + Data + ETX + Checksum + CR + LF
        // Frame number is ASCII digit (0x30-0x39)
        // STX is at index 1
        // ETX is at frame.len() - 4
        // Checksum is at frame.len() - 3
        // CR is at frame.len() - 2
        // LF is at frame.len() - 1

        let mut sum = 0u8;
        let start_idx = 0; // Start from frame number (including it)
        let end_idx = frame.len() - 3; // End at ETX (inclusive)

        for &byte in &frame[start_idx..end_idx] {
            sum = sum.wrapping_add(byte);
        }

        let expected_checksum = sum % 8;
        let actual_checksum = frame[frame.len() - 3]; // Checksum byte

        log::debug!(
            "Checksum validation: sum={}, expected={}, actual={}, valid={}",
            sum,
            expected_checksum,
            actual_checksum,
            expected_checksum == actual_checksum
        );

        expected_checksum == actual_checksum
    }

    /// Extracts frame data from ASTM frame
    fn extract_frame_data(frame: &[u8]) -> Result<Vec<u8>, String> {
        if frame.len() < 6 {
            return Err("Frame too short".to_string());
        }

        // Find STX and ETX positions
        let stx_pos = frame.iter().position(|&b| b == ASTM_STX);
        let etx_pos = frame.iter().position(|&b| b == ASTM_ETX);

        match (stx_pos, etx_pos) {
            (Some(stx), Some(etx)) if stx < etx => {
                // Extract data between STX and ETX (exclusive)
                let start_idx = stx + 1; // After STX
                let end_idx = etx; // Before ETX

                let extracted_data = frame[start_idx..end_idx].to_vec();

                // Verify frame ends with CR and LF
                if frame.len() >= 2 {
                    let cr_pos = frame.len() - 2;
                    let lf_pos = frame.len() - 1;

                    if frame[cr_pos] != ASTM_CR || frame[lf_pos] != ASTM_LF {
                        log::warn!(
                            "Frame does not end with CR+LF: CR=0x{:02X}, LF=0x{:02X}",
                            frame[cr_pos],
                            frame[lf_pos]
                        );
                    }
                }

                Ok(extracted_data)
            }
            _ => {
                log::error!("Could not find STX or ETX in frame: {:?}", frame);
                Err("Invalid frame structure: missing STX or ETX".to_string())
            }
        }
    }

    /// Parses ASTM record type
    fn parse_record_type(frame_data: &[u8]) -> Result<String, String> {
        if frame_data.is_empty() {
            return Err("Empty frame data".to_string());
        }

        let first_char: char = frame_data[1] as char;
        let record_type = match first_char {
            'H' => "Header",
            'P' => "Patient",
            'O' => "Order",
            'R' => "Result",
            'C' => "Comment",
            'Q' => "Request",
            'L' => "Terminator",
            _ => "Unknown",
        };

        log::debug!("Parsing record type: {}", record_type);

        Ok(record_type.to_string())
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

    /// Parses a patient record from ASTM data
    fn parse_patient_record(frame_data: &[u8]) -> Result<PatientData, String> {
        let data_str = String::from_utf8_lossy(frame_data);
        let fields: Vec<&str> = data_str.split('|').collect();

        if fields.len() < 2 {
            return Err("Invalid patient record format".to_string());
        }

        // Parse patient name (field 6) - format: LastName^FirstName^MiddleName^Title
        let name_parts: Vec<&str> = fields.get(6).unwrap_or(&"").split('^').collect();
        let name = if name_parts.len() >= 2 {
            format!(
                "{} {}",
                name_parts.get(1).unwrap_or(&""),
                name_parts.get(0).unwrap_or(&"")
            )
        } else {
            fields.get(6).unwrap_or(&"").to_string()
        };

        Ok(PatientData {
            id: fields.get(3).unwrap_or(&"").to_string(),
            name,
            birth_date: fields.get(8).map(|s| s.to_string()),
            sex: fields.get(9).map(|s| s.to_string()),
            address: fields.get(11).map(|s| s.to_string()),
            telephone: fields.get(13).map(|s| s.to_string()),
            physicians: fields.get(14).map(|s| s.to_string()),
            height: fields.get(17).map(|s| s.to_string()),
            weight: fields.get(18).map(|s| s.to_string()),
        })
    }

    /// Parses a result record from ASTM data
    fn parse_result_record(frame_data: &[u8]) -> Result<TestResult, String> {
        let data_str = String::from_utf8_lossy(frame_data);
        let fields: Vec<&str> = data_str.split('|').collect();

        if fields.len() < 4 {
            return Err("Invalid result record format".to_string());
        }

        // Parse test ID (field 3) - format: ^^^TEST_NAME
        let test_id_parts: Vec<&str> = fields.get(3).unwrap_or(&"").split('^').collect();
        let test_name = test_id_parts.last().unwrap_or(&"").to_string();

        // Parse reference range (field 6) - format: lower^upper
        let reference_range = fields.get(6).and_then(|range_str| {
            if !range_str.is_empty() {
                let parts: Vec<&str> = range_str.split('^').collect();
                if parts.len() >= 2 {
                    Some(format!("{}-{}", parts[0], parts[1]))
                } else {
                    Some(range_str.to_string())
                }
            } else {
                None
            }
        });

        // Parse flags (field 7)
        let flags = fields
            .get(7)
            .map(|flag_str| {
                if !flag_str.is_empty() {
                    vec![flag_str.to_string()]
                } else {
                    vec![]
                }
            })
            .unwrap_or_default();

        let now = Utc::now();
        Ok(TestResult {
            id: format!("result_{}", now.timestamp()),
            test_id: test_name.clone(),
            sample_id: fields.get(2).unwrap_or(&"").to_string(), // Sequence number as sample ID
            value: fields.get(4).unwrap_or(&"").to_string(),
            units: fields.get(5).map(|s| s.to_string()),
            reference_range,
            flags,
            status: fields.get(9).unwrap_or(&"F").to_string(), // Result status (F=Final, P=Preliminary, C=Correction)
            completed_date_time: Some(now),
            analyzer_id: None, // Will be set by the caller
            created_at: now,
            updated_at: now,
        })
    }
}
