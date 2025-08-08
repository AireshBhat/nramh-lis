use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Serial,
    TcpIp,
}

impl ToString for ConnectionType {
    fn to_string(&self) -> String {
        match self {
            ConnectionType::Serial => "SERIAL".to_string(),
            ConnectionType::TcpIp => "TCP/IP".to_string(),
        }
    }
}

impl From<&str> for ConnectionType {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "SERIAL" => ConnectionType::Serial,
            _ => ConnectionType::TcpIp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalyzerStatus {
    Active,
    Inactive,
    Maintenance,
}

impl ToString for AnalyzerStatus {
    fn to_string(&self) -> String {
        match self {
            AnalyzerStatus::Active => "ACTIVE".to_string(),
            AnalyzerStatus::Inactive => "INACTIVE".to_string(),
            AnalyzerStatus::Maintenance => "MAINTENANCE".to_string(),
        }
    }
}

impl From<&str> for AnalyzerStatus {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "INACTIVE" => AnalyzerStatus::Inactive,
            "MAINTENANCE" => AnalyzerStatus::Maintenance,
            _ => AnalyzerStatus::Active,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Protocol {
    Astm,
    Hl7,
    Hl7V24, // HL7 version 2.4 for BF-6900 Hematology analyzer
    Hl7V231, // HL7 version 2.3.1 for BF-6900 Hematology analyzer (CQ 5 Plus)
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::Astm => "ASTM".to_string(),
            Protocol::Hl7 => "HL7".to_string(),
            Protocol::Hl7V24 => "HL7_V24".to_string(),
            Protocol::Hl7V231 => "HL7_V231".to_string(),
        }
    }
}

impl From<&str> for Protocol {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "HL7" => Protocol::Hl7,
            "HL7_V24" => Protocol::Hl7V24,
            "HL7_V231" => Protocol::Hl7V231,
            _ => Protocol::Astm,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analyzer {
    pub id: String,
    pub name: String,
    pub model: String,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub connection_type: ConnectionType,
    pub ip_address: Option<String>,
    pub port: Option<u16>,
    pub com_port: Option<String>,
    pub baud_rate: Option<u32>,
    /// External IP address for connecting to external LIS instruments
    /// Used when this analyzer needs to initiate connections to other systems
    pub external_ip: Option<String>,
    /// External port for connecting to external LIS instruments  
    /// Used when this analyzer needs to initiate connections to other systems
    pub external_port: Option<u16>,
    pub protocol: Protocol,
    pub status: AnalyzerStatus,
    pub activate_on_start: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
