use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceRange {
    pub lower_limit: Option<f64>,
    pub upper_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultFlags {
    pub abnormal_flag: Option<String>,
    pub nature_of_abnormality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultStatus {
    Correction,  // "C" - Correction of previously transmitted results
    Final,       // "F" - Final results
    Preliminary, // "P" - Preliminary results
}

impl From<&str> for ResultStatus {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "C" => ResultStatus::Correction,
            "P" => ResultStatus::Preliminary,
            _ => ResultStatus::Final,
        }
    }
}

impl ToString for ResultStatus {
    fn to_string(&self) -> String {
        match self {
            ResultStatus::Correction => "C".to_string(),
            ResultStatus::Final => "F".to_string(),
            ResultStatus::Preliminary => "P".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResultMetadata {
    pub sequence_number: u32,
    pub instrument: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: String,
    pub test_id: String,       // Universal Test ID (e.g., ^^^ALB)
    pub sample_id: String,     // Reference to the sample
    pub value: String,         // Measurement value
    pub units: Option<String>, // ISO 2955 units (e.g., g/dL, IU/L)
    pub reference_range: Option<ReferenceRange>, // Reference range information
    pub flags: Option<ResultFlags>, // Result flags
    pub status: ResultStatus,  // Result status
    pub completed_date_time: Option<DateTime<Utc>>, // When test was completed
    pub metadata: TestResultMetadata, // Additional metadata
    pub analyzer_id: Option<String>, // Reference to the analyzer that produced this result
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
