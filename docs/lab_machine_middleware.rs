use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Core traits and types for the lab machine middleware system
/// Following systems thinking: honor information flow, respect complexity

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    Astm,      // MERIL AutoQuant
    Hl7V24,    // Afinion 2, BF-6900
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transport {
    Serial { port: String, baud_rate: u32 },
    TcpIp { host: String, port: u16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    // ASTM Types
    PatientInfo,
    TestOrder,
    Result,
    Comment,
    Request,
    
    // HL7 Types
    ORU_R01,  // Unsolicited results
    OUL_R21,  // Quality control
    ACK,      // Acknowledgment
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub test_name: String,
    pub value: ResultValue,
    pub unit: Option<String>,
    pub reference_range: Option<String>,
    pub flags: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum ResultValue {
    Numeric(f64),
    Text(String),
    Binary(Vec<u8>),
    OutOfRange { flag: String, limit_value: Option<f64> },
    Uncomputable,  // "---" values
}

#[derive(Debug, Clone)]
pub struct PatientInfo {
    pub patient_id: String,
    pub name: Option<String>,
    pub birth_date: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub department: Option<String>,
    pub physician: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LabMessage {
    pub message_id: String,
    pub message_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub patient_info: Option<PatientInfo>,
    pub test_results: Vec<TestResult>,
    pub raw_data: Vec<u8>,
    pub protocol: Protocol,
}

/// Communication status tracking - feedback systems principle
#[derive(Debug, Clone)]
pub enum CommunicationStatus {
    Idle,
    Establishing,
    Transferring,
    Terminating,
    Error(String),
}

/// Protocol-specific handler trait - respecting different system languages
#[async_trait]
pub trait ProtocolHandler: Send + Sync {
    async fn parse_message(&self, data: &[u8]) -> Result<LabMessage, Box<dyn std::error::Error>>;
    async fn serialize_message(&self, message: &LabMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    async fn handle_acknowledgment(&self, ack_data: &[u8]) -> Result<bool, Box<dyn std::error::Error>>;
    fn get_protocol(&self) -> Protocol;
}

/// ASTM Protocol Handler - for MERIL AutoQuant
pub struct AstmHandler {
    checksum_validation: bool,
}

impl AstmHandler {
    pub fn new(checksum_validation: bool) -> Self {
        Self { checksum_validation }
    }
    
    fn calculate_checksum(&self, frame: &[u8]) -> u8 {
        frame.iter().fold(0u8, |acc, &byte| acc.wrapping_add(byte)) % 8
    }
}

#[async_trait]
impl ProtocolHandler for AstmHandler {
    async fn parse_message(&self, data: &[u8]) -> Result<LabMessage, Box<dyn std::error::Error>> {
        // Parse ASTM format: STX + frame + ETX + checksum + CR + LF
        // This is simplified - real implementation would handle full ASTM parsing
        
        let message_id = format!("ASTM-{}", chrono::Utc::now().timestamp());
        let timestamp = chrono::Utc::now();
        
        // Extract record type from first character after STX
        let message_type = match data.get(1) {
            Some(b'P') => MessageType::PatientInfo,
            Some(b'O') => MessageType::TestOrder,
            Some(b'R') => MessageType::Result,
            Some(b'C') => MessageType::Comment,
            Some(b'Q') => MessageType::Request,
            _ => return Err("Unknown ASTM record type".into()),
        };
        
        Ok(LabMessage {
            message_id,
            message_type,
            timestamp,
            patient_info: None, // Would parse from P record
            test_results: vec![], // Would parse from R records
            raw_data: data.to_vec(),
            protocol: Protocol::Astm,
        })
    }
    
    async fn serialize_message(&self, message: &LabMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Serialize to ASTM format with proper framing
        let mut result = Vec::new();
        result.push(0x02); // STX
        
        // Add message content based on type
        match message.message_type {
            MessageType::Result => {
                result.extend_from_slice(b"R|1|"); // Basic R record start
                // Add test results...
            }
            _ => return Err("Unsupported message type for ASTM".into()),
        }
        
        result.push(0x03); // ETX
        
        if self.checksum_validation {
            let checksum = self.calculate_checksum(&result[1..result.len()-1]);
            result.extend_from_slice(&format!("{:02X}", checksum).as_bytes());
        }
        
        result.extend_from_slice(b"\r\n");
        Ok(result)
    }
    
    async fn handle_acknowledgment(&self, ack_data: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        match ack_data.get(0) {
            Some(0x06) => Ok(true),  // ACK
            Some(0x15) => Ok(false), // NAK
            _ => Err("Invalid acknowledgment".into()),
        }
    }
    
    fn get_protocol(&self) -> Protocol {
        Protocol::Astm
    }
}

/// HL7 Protocol Handler - for Afinion 2 and BF-6900
pub struct Hl7Handler {
    device_name: String,
    version: String,
}

impl Hl7Handler {
    pub fn new(device_name: String, version: String) -> Self {
        Self { device_name, version }
    }
    
    fn parse_hl7_segment(&self, segment: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = segment.split('|').collect();
        let mut fields = HashMap::new();
        
        if !parts.is_empty() {
            fields.insert("segment_type".to_string(), parts[0].to_string());
            for (i, part) in parts.iter().enumerate().skip(1) {
                fields.insert(format!("field_{}", i), part.to_string());
            }
        }
        
        Ok(fields)
    }
}

#[async_trait]
impl ProtocolHandler for Hl7Handler {
    async fn parse_message(&self, data: &[u8]) -> Result<LabMessage, Box<dyn std::error::Error>> {
        // Parse HL7 format: <VT>segments<FS><CR>
        let message_str = String::from_utf8_lossy(data);
        let segments: Vec<&str> = message_str.lines().collect();
        
        let mut patient_info = None;
        let mut test_results = Vec::new();
        let mut message_type = MessageType::ORU_R01;
        let message_id = format!("HL7-{}", chrono::Utc::now().timestamp());
        
        for segment in segments {
            if segment.starts_with("MSH") {
                let fields = self.parse_hl7_segment(segment)?;
                if let Some(msg_type) = fields.get("field_9") {
                    message_type = match msg_type.as_str() {
                        "ORU^R01" => MessageType::ORU_R01,
                        "OUL^R21" => MessageType::OUL_R21,
                        "ACK" => MessageType::ACK,
                        _ => MessageType::ORU_R01,
                    };
                }
            } else if segment.starts_with("PID") {
                let fields = self.parse_hl7_segment(segment)?;
                patient_info = Some(PatientInfo {
                    patient_id: fields.get("field_3").unwrap_or(&String::new()).clone(),
                    name: fields.get("field_5").cloned(),
                    birth_date: None, // Would parse from field_7
                    gender: fields.get("field_8").cloned(),
                    department: None,
                    physician: None,
                });
            } else if segment.starts_with("OBX") {
                let fields = self.parse_hl7_segment(segment)?;
                if let (Some(test_id), Some(value_str)) = (fields.get("field_3"), fields.get("field_5")) {
                    let value = if value_str.starts_with('<') || value_str.starts_with('>') {
                        ResultValue::OutOfRange { 
                            flag: value_str.chars().next().unwrap().to_string(),
                            limit_value: value_str[1..].parse().ok()
                        }
                    } else if value_str == "---" {
                        ResultValue::Uncomputable
                    } else if let Ok(num_val) = value_str.parse::<f64>() {
                        ResultValue::Numeric(num_val)
                    } else {
                        ResultValue::Text(value_str.clone())
                    };
                    
                    test_results.push(TestResult {
                        test_id: test_id.clone(),
                        test_name: test_id.clone(),
                        value,
                        unit: fields.get("field_6").cloned(),
                        reference_range: fields.get("field_7").cloned(),
                        flags: vec![],
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }
        
        Ok(LabMessage {
            message_id,
            message_type,
            timestamp: chrono::Utc::now(),
            patient_info,
            test_results,
            raw_data: data.to_vec(),
            protocol: Protocol::Hl7V24,
        })
    }
    
    async fn serialize_message(&self, message: &LabMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut result = Vec::new();
        result.push(0x0B); // VT
        
        // MSH segment
        let msh = format!(
            "MSH|^~\\&|{}||||{}||ORU^R01|{}|P|2.4|||AL|NE||UTF-8\r",
            self.device_name,
            chrono::Utc::now().format("%Y%m%d%H%M%S"),
            message.message_id
        );
        result.extend_from_slice(msh.as_bytes());
        
        // PID segment if patient info available
        if let Some(patient) = &message.patient_info {
            let pid = format!("PID||{}|||{}|||{}\r", 
                patient.patient_id,
                patient.name.as_ref().unwrap_or(&String::new()),
                patient.gender.as_ref().unwrap_or(&String::new())
            );
            result.extend_from_slice(pid.as_bytes());
        }
        
        // OBR segment
        result.extend_from_slice(b"OBR||1||Test Results\r");
        
        // OBX segments for results
        for (i, test_result) in message.test_results.iter().enumerate() {
            let value_str = match &test_result.value {
                ResultValue::Numeric(n) => n.to_string(),
                ResultValue::Text(t) => t.clone(),
                ResultValue::OutOfRange { flag, limit_value } => {
                    format!("{}{}", flag, limit_value.map_or(String::new(), |v| v.to_string()))
                },
                ResultValue::Uncomputable => "---".to_string(),
                ResultValue::Binary(_) => "<<BINARY_DATA>>".to_string(),
            };
            
            let obx = format!(
                "OBX|{}|NM|{}||{}|{}|||||F\r",
                i + 1,
                test_result.test_id,
                value_str,
                test_result.unit.as_ref().unwrap_or(&String::new())
            );
            result.extend_from_slice(obx.as_bytes());
        }
        
        result.push(0x1C); // FS
        result.push(0x0D); // CR
        
        Ok(result)
    }
    
    async fn handle_acknowledgment(&self, ack_data: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let ack_str = String::from_utf8_lossy(ack_data);
        Ok(ack_str.contains("MSA|AA") || ack_str.contains("MSA|CA"))
    }
    
    fn get_protocol(&self) -> Protocol {
        Protocol::Hl7V24
    }
}

/// Main LabMachine middleware struct - the system orchestrator
/// Embodies systems thinking: whole system perspective, feedback loops, complexity handling
pub struct LabMachine {
    pub machine_id: String,
    pub machine_type: String,
    pub protocol_handler: Arc<dyn ProtocolHandler>,
    pub transport: Transport,
    pub status: Arc<RwLock<CommunicationStatus>>,
    pub message_sender: mpsc::UnboundedSender<LabMessage>,
    pub message_receiver: Option<mpsc::UnboundedReceiver<LabMessage>>,
    pub error_handler: Option<Box<dyn Fn(Box<dyn std::error::Error>) + Send + Sync>>,
    pub metrics: Arc<RwLock<SystemMetrics>>,
}

#[derive(Debug, Default)]
pub struct SystemMetrics {
    pub messages_processed: u64,
    pub errors_count: u64,
    pub last_communication: Option<DateTime<Utc>>,
    pub average_response_time: Option<std::time::Duration>,
    pub connection_uptime: std::time::Duration,
}

impl LabMachine {
    pub fn new(
        machine_id: String,
        machine_type: String,
        protocol_handler: Arc<dyn ProtocolHandler>,
        transport: Transport,
    ) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Self {
            machine_id,
            machine_type,
            protocol_handler,
            transport,
            status: Arc::new(RwLock::new(CommunicationStatus::Idle)),
            message_sender: sender,
            message_receiver: Some(receiver),
            error_handler: None,
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        }
    }
    
    /// Create ASTM-based lab machine (like MERIL AutoQuant)
    pub fn new_astm_machine(
        machine_id: String,
        machine_type: String,
        transport: Transport,
        checksum_validation: bool,
    ) -> Self {
        let handler = Arc::new(AstmHandler::new(checksum_validation));
        Self::new(machine_id, machine_type, handler, transport)
    }
    
    /// Create HL7-based lab machine (like Afinion 2, BF-6900)
    pub fn new_hl7_machine(
        machine_id: String,
        machine_type: String,
        transport: Transport,
        device_name: String,
    ) -> Self {
        let handler = Arc::new(Hl7Handler::new(device_name, "2.4".to_string()));
        Self::new(machine_id, machine_type, handler, transport)
    }
    
    /// Set error handler for system resilience
    pub fn with_error_handler<F>(mut self, handler: F) -> Self 
    where
        F: Fn(Box<dyn std::error::Error>) + Send + Sync + 'static,
    {
        self.error_handler = Some(Box::new(handler));
        self
    }
    
    /// Start the middleware system - listen to the wisdom of the system
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        *self.status.write().await = CommunicationStatus::Establishing;
        
        // Here would be actual transport connection logic
        match &self.transport {
            Transport::Serial { port, baud_rate } => {
                println!("Connecting to serial port {} at {} baud", port, baud_rate);
                // Serial port connection logic
            }
            Transport::TcpIp { host, port } => {
                println!("Connecting to TCP/IP {}:{}", host, port);
                // TCP connection logic
            }
        }
        
        *self.status.write().await = CommunicationStatus::Idle;
        Ok(())
    }
    
    /// Process incoming message - honor and distribute information
    pub async fn process_message(&self, raw_data: &[u8]) -> Result<LabMessage, Box<dyn std::error::Error>> {
        let start_time = std::time::Instant::now();
        
        let message = self.protocol_handler.parse_message(raw_data).await?;
        
        // Update metrics - pay attention to what is important
        {
            let mut metrics = self.metrics.write().await;
            metrics.messages_processed += 1;
            metrics.last_communication = Some(chrono::Utc::now());
            metrics.average_response_time = Some(start_time.elapsed());
        }
        
        // Send to message queue for HIS processing
        if let Err(e) = self.message_sender.send(message.clone()) {
            if let Some(error_handler) = &self.error_handler {
                error_handler(Box::new(e));
            }
        }
        
        Ok(message)
    }
    
    /// Send message to HIS - go for the good of the whole
    pub async fn send_to_his(&self, message: &LabMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.protocol_handler.serialize_message(message).await
    }
    
    /// Get system status - expand time horizons, stay humble
    pub async fn get_status(&self) -> CommunicationStatus {
        self.status.read().await.clone()
    }
    
    /// Get system metrics - celebrate complexity
    pub async fn get_metrics(&self) -> SystemMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Health check - locate responsibility in the system
    pub async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let status = self.get_status().await;
        match status {
            CommunicationStatus::Error(_) => Ok(false),
            _ => Ok(true),
        }
    }
}

/// Factory for creating different lab machine instances
/// Defy the disciplines - bridge different protocols and systems
pub struct LabMachineFactory;

impl LabMachineFactory {
    pub fn create_meril_autoquant(
        machine_id: String,
        transport: Transport,
    ) -> LabMachine {
        LabMachine::new_astm_machine(
            machine_id,
            "MERIL AutoQuant".to_string(),
            transport,
            true, // Enable checksum validation
        )
    }
    
    pub fn create_afinion2(
        machine_id: String,
        transport: Transport,
    ) -> LabMachine {
        LabMachine::new_hl7_machine(
            machine_id,
            "Afinion 2".to_string(),
            transport,
            "Afinion 2 Analyzer".to_string(),
        )
    }
    
    pub fn create_bf6900(
        machine_id: String,
        transport: Transport,
    ) -> LabMachine {
        LabMachine::new_hl7_machine(
            machine_id,
            "BF-6900".to_string(),
            transport,
            "BF-6900".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_lab_machine_creation() {
        let transport = Transport::TcpIp {
            host: "192.168.1.100".to_string(),
            port: 9100,
        };
        
        let machine = LabMachineFactory::create_afinion2(
            "AFINION-001".to_string(),
            transport,
        );
        
        assert_eq!(machine.machine_id, "AFINION-001");
        assert_eq!(machine.machine_type, "Afinion 2");
    }
    
    #[tokio::test]
    async fn test_hl7_message_parsing() {
        let handler = Hl7Handler::new("Test Device".to_string(), "2.4".to_string());
        let sample_data = b"MSH|^~\\&|Test Device||||20210101120000||ORU^R01|123|P|2.4\r\nPID||12345|||John^Doe|||M\r\nOBX|1|NM|GLU||150|mg/dL|||||F\r\n";
        
        let result = handler.parse_message(sample_data).await;
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.protocol, Protocol::Hl7V24);
        assert_eq!(message.message_type, MessageType::ORU_R01);
        assert!(message.patient_info.is_some());
        assert_eq!(message.test_results.len(), 1);
    }
}