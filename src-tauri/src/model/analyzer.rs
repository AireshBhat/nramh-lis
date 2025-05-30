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
    pub status: AnalyzerStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
