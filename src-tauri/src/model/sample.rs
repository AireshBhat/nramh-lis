use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerType {
    Tube10ml,   // "1" in protocol
    Tube5to7ml, // "3" in protocol
    Other(String),
}

impl From<&str> for ContainerType {
    fn from(s: &str) -> Self {
        match s {
            "1" => ContainerType::Tube10ml,
            "3" => ContainerType::Tube5to7ml,
            other => ContainerType::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub number: String,
    pub container_type: ContainerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub date_time: Option<DateTime<Utc>>,
    pub collector_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptionInfo {
    pub date_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SampleType {
    Blood,
    Urine,
    Serum,
    Plasma,
    Csf,
    Other(String),
}

impl From<&str> for SampleType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blood" => SampleType::Blood,
            "urine" => SampleType::Urine,
            "serum" => SampleType::Serum,
            "plasma" => SampleType::Plasma,
            "csf" => SampleType::Csf,
            other => SampleType::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SampleStatus {
    Pending,
    InProgress,
    Completed,
    Canceled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub id: String,                            // Specimen ID
    pub container_info: Option<ContainerInfo>, // Container information
    pub collection: Option<CollectionInfo>,    // Collection information
    pub reception: Option<ReceptionInfo>,      // Reception information
    pub sample_type: SampleType,               // Sample type (Blood, Urine, etc.)
    pub status: SampleStatus,                  // Sample processing status
    pub position: Option<String>,              // Position in analyzer
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
