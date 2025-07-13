use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio::time::timeout;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tauri::Runtime;

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
        test_results: Vec<TestResult>,
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
    pub test_id: String,
    pub test_name: String,
    pub value: String,
    pub unit: Option<String>,
    pub reference_range: Option<String>,
    pub flags: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

// ============================================================================
// ASTM PROTOCOL CONSTANTS
// ============================================================================

const ASTM_ENQ: u8 = 0x05;  // ENQ - Enquiry
const ASTM_ACK: u8 = 0x06;  // ACK - Acknowledgment
const ASTM_NAK: u8 = 0x15;  // NAK - Negative Acknowledgment
const ASTM_EOT: u8 = 0x04;  // EOT - End of Transmission
const ASTM_STX: u8 = 0x02;  // STX - Start of Text
const ASTM_ETX: u8 = 0x03;  // ETX - End of Text
const ASTM_ETB: u8 = 0x17;  // ETB - End of Transmission Block
const ASTM_CR: u8 = 0x0D;   // CR - Carriage Return
const ASTM_LF: u8 = 0x0A;   // LF - Line Feed

// ============================================================================
// CONNECTION STATE
// ============================================================================

#[derive(Debug, Clone)]
pub enum ConnectionState {
    WaitingForEnq,
    WaitingForFrame,
    ProcessingFrame,
    Complete,
}

#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
    pub remote_addr: SocketAddr,
    pub state: ConnectionState,
    pub frame_buffer: Vec<u8>,
    pub analyzer_id: String,
}

// ============================================================================
// MAIN SERVICE
// ============================================================================

pub struct AutoQuantMerilService<R: Runtime> {
    /// Analyzer configuration
    analyzer: Analyzer,
    /// TCP listener for incoming connections
    listener: Option<TcpListener>,
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
            analyzer,
            listener: None,
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            is_running: Arc::new(RwLock::new(false)),
            store,
        }
    }

    /// Starts the service
    pub async fn start(&mut self) -> Result<(), String> {
        let port = self.analyzer.port.ok_or("No port configured")?;
        let bind_addr = format!("0.0.0.0:{}", port);
        
        log::info!("Starting AutoQuantMeril service on {}", bind_addr);
        
        // Create TCP listener
        let listener = TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| format!("Failed to bind to {}: {}", bind_addr, e))?;
        
        self.listener = Some(listener);
        *self.is_running.write().await = true;
        
        log::info!("AutoQuantMeril service started successfully on port {}", port);
        
        // Start the connection handler
        self.handle_connections().await;
        
        Ok(())
    }

    /// Stops the service
    pub async fn stop(&mut self) -> Result<(), String> {
        log::info!("Stopping AutoQuantMeril service");
        
        *self.is_running.write().await = false;
        
        // Close all connections
        let mut connections = self.connections.write().await;
        for (analyzer_id, mut connection) in connections.drain() {
            if let Err(e) = connection.stream.shutdown().await {
                log::warn!("Error shutting down connection for {}: {}", analyzer_id, e);
            }
        }
        
        self.listener = None;
        
        log::info!("AutoQuantMeril service stopped");
        Ok(())
    }

    /// Main connection handling loop
    async fn handle_connections(&self) {
        let listener = match &self.listener {
            Some(l) => l,
            None => {
                log::error!("No TCP listener available");
                return;
            }
        };

        let is_running = self.is_running.clone();
        let connections = self.connections.clone();
        let event_sender = self.event_sender.clone();
        let analyzer_id = self.analyzer.id.clone();

        loop {
            // Check if service should stop
            if !*is_running.read().await {
                break;
            }

            // Accept incoming connections
            match timeout(Duration::from_secs(1), listener.accept()).await {
                Ok(Ok((stream, addr))) => {
                    log::info!("New connection from {}", addr);
                    
                    let connection = Connection {
                        stream,
                        remote_addr: addr,
                        state: ConnectionState::WaitingForEnq,
                        frame_buffer: Vec::new(),
                        analyzer_id: analyzer_id.clone(),
                    };
                    
                    // Store connection
                    connections.write().await.insert(analyzer_id.clone(), connection);
                    
                    // Send connection event
                    let _ = event_sender.send(MerilEvent::AnalyzerConnected {
                        analyzer_id: analyzer_id.clone(),
                        remote_addr: addr.to_string(),
                        timestamp: Utc::now(),
                    }).await;
                    
                    // Handle the connection
                    let connections_clone = connections.clone();
                    let event_sender_clone = event_sender.clone();
                    let analyzer_id_clone = analyzer_id.clone();
                    
                    tokio::spawn(async move {
                        Self::handle_connection(connections_clone, event_sender_clone, analyzer_id_clone).await;
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
                        
                        let _ = event_sender.send(MerilEvent::Error {
                            analyzer_id: analyzer_id.clone(),
                            error: e,
                            timestamp: Utc::now(),
                        }).await;
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
        let _ = event_sender.send(MerilEvent::AnalyzerDisconnected {
            analyzer_id,
            timestamp: Utc::now(),
        }).await;
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
                        connection.stream.write_all(&[ASTM_ACK]).await
                            .map_err(|e| format!("Failed to send ACK: {}", e))?;
                        
                        connection.state = ConnectionState::WaitingForFrame;
                        log::debug!("Received ENQ, sent ACK, waiting for frame");
                    }
                }
                ConnectionState::WaitingForFrame => {
                    if byte == ASTM_STX {
                        connection.frame_buffer.clear();
                        connection.frame_buffer.push(byte);
                        connection.state = ConnectionState::ProcessingFrame;
                        log::debug!("Received STX, processing frame");
                    } else if byte == ASTM_EOT {
                        // End of transmission
                        connection.state = ConnectionState::Complete;
                        log::info!("Received EOT, transmission complete");
                        
                        // Process complete message
                        Self::process_complete_message(connection, event_sender).await?;
                        
                        // Send ACK for EOT
                        connection.stream.write_all(&[ASTM_ACK]).await
                            .map_err(|e| format!("Failed to send ACK for EOT: {}", e))?;
                        
                        connection.state = ConnectionState::WaitingForEnq;
                    }
                }
                ConnectionState::ProcessingFrame => {
                    connection.frame_buffer.push(byte);
                    
                    if byte == ASTM_ETX || byte == ASTM_ETB {
                        // End of frame
                        if let Err(e) = Self::process_frame(connection, event_sender).await {
                            // Send NAK on error
                            connection.stream.write_all(&[ASTM_NAK]).await
                                .map_err(|e| format!("Failed to send NAK: {}", e))?;
                            return Err(e);
                        }
                        
                        // Send ACK
                        connection.stream.write_all(&[ASTM_ACK]).await
                            .map_err(|e| format!("Failed to send ACK: {}", e))?;
                        
                        connection.state = ConnectionState::WaitingForFrame;
                    }
                }
                ConnectionState::Complete => {
                    // Should not reach here
                    log::warn!("Unexpected data in Complete state");
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
        // Validate checksum
        if !Self::validate_checksum(&connection.frame_buffer) {
            return Err("Invalid checksum".to_string());
        }
        
        // Extract frame data (remove STX, ETX/ETB, checksum, CR, LF)
        let frame_data = Self::extract_frame_data(&connection.frame_buffer)?;
        
        // Parse ASTM record
        let record_type = Self::parse_record_type(&frame_data)?;
        
        log::debug!("Processed ASTM frame: {} - {}", record_type, String::from_utf8_lossy(&frame_data));
        
        // Send event
        let _ = event_sender.send(MerilEvent::AstmMessageReceived {
            analyzer_id: connection.analyzer_id.clone(),
            message_type: record_type,
            raw_data: String::from_utf8_lossy(&frame_data).to_string(),
            timestamp: Utc::now(),
        }).await;
        
        Ok(())
    }

    /// Processes complete ASTM message
    async fn process_complete_message(
        connection: &mut Connection,
        event_sender: &mpsc::Sender<MerilEvent>,
    ) -> Result<(), String> {
        // For now, just log the complete message
        // In a full implementation, this would parse all records and create TestResults
        log::info!("Processing complete ASTM message from {}", connection.remote_addr);
        
        // Mock test results for demonstration
        let test_results = vec![
            TestResult {
                test_id: "GLU".to_string(),
                test_name: "Glucose".to_string(),
                value: "95".to_string(),
                unit: Some("mg/dL".to_string()),
                reference_range: Some("70-100".to_string()),
                flags: vec![],
                timestamp: Utc::now(),
            }
        ];
        
        let _ = event_sender.send(MerilEvent::LabResultProcessed {
            analyzer_id: connection.analyzer_id.clone(),
            patient_id: Some("PAT001".to_string()),
            test_results,
            timestamp: Utc::now(),
        }).await;
        
        Ok(())
    }

    /// Validates ASTM frame checksum
    fn validate_checksum(frame: &[u8]) -> bool {
        if frame.len() < 4 {
            return false;
        }
        
        // Simple checksum validation (modulo 8 of sum)
        let mut sum = 0u8;
        let start_idx = 1; // Skip STX
        let end_idx = frame.len() - 4; // Before ETX/ETB, checksum, CR, LF
        
        for &byte in &frame[start_idx..end_idx] {
            sum = sum.wrapping_add(byte);
        }
        
        let expected_checksum = sum % 8;
        let actual_checksum = frame[frame.len() - 3]; // Checksum byte
        
        expected_checksum == actual_checksum
    }

    /// Extracts frame data from ASTM frame
    fn extract_frame_data(frame: &[u8]) -> Result<Vec<u8>, String> {
        if frame.len() < 4 {
            return Err("Frame too short".to_string());
        }
        
        // Remove STX, ETX/ETB, checksum, CR, LF
        let start_idx = 1; // After STX
        let end_idx = frame.len() - 4; // Before ETX/ETB, checksum, CR, LF
        
        Ok(frame[start_idx..end_idx].to_vec())
    }

    /// Parses ASTM record type
    fn parse_record_type(frame_data: &[u8]) -> Result<String, String> {
        if frame_data.is_empty() {
            return Err("Empty frame data".to_string());
        }
        
        let first_char = frame_data[0] as char;
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
    pub fn get_analyzer_config(&self) -> &Analyzer {
        &self.analyzer
    }
} 