use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Uploaded,
    Failed,
}

impl ToString for UploadStatus {
    fn to_string(&self) -> String {
        match self {
            UploadStatus::Pending => "PENDING".to_string(),
            UploadStatus::Uploading => "UPLOADING".to_string(),
            UploadStatus::Uploaded => "UPLOADED".to_string(),
            UploadStatus::Failed => "FAILED".to_string(),
        }
    }
}

impl From<&str> for UploadStatus {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "UPLOADING" => UploadStatus::Uploading,
            "UPLOADED" => UploadStatus::Uploaded,
            "FAILED" => UploadStatus::Failed,
            _ => UploadStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultUploadStatus {
    pub id: String,
    pub result_id: String,
    pub external_system_id: String,
    pub status: UploadStatus,
    pub upload_date: Option<DateTime<Utc>>,
    pub response_code: Option<String>,
    pub response_message: Option<String>,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
