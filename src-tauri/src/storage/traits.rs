use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::model::{
    Analyzer, Patient, TestResult, Sample, TestOrder, ResultUploadStatus,
    analyzer::AnalyzerStatus, sample::SampleStatus, upload::UploadStatus, result::ResultStatus
};

/// Base repository trait for all entities
#[async_trait]
pub trait Repository<T> {
    type Error: std::error::Error + Send + Sync;
    type Id;

    async fn create(&self, entity: &T) -> Result<Self::Id, Self::Error>;
    async fn find_by_id(&self, id: Self::Id) -> Result<Option<T>, Self::Error>;
    async fn update(&self, id: Self::Id, entity: &T) -> Result<(), Self::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), Self::Error>;
    async fn list(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<T>, Self::Error>;
}

/// Analyzer-specific repository operations
#[async_trait]
pub trait AnalyzerRepository: Repository<Analyzer> {
    async fn find_by_status(&self, status: AnalyzerStatus) -> Result<Vec<Analyzer>, Self::Error>;
    async fn find_by_connection_type(&self, connection_type: &str) -> Result<Vec<Analyzer>, Self::Error>;
    async fn find_active_analyzers(&self) -> Result<Vec<Analyzer>, Self::Error>;
    async fn update_status(&self, id: &str, status: AnalyzerStatus) -> Result<(), Self::Error>;
    async fn find_by_serial_number(&self, serial_number: &str) -> Result<Option<Analyzer>, Self::Error>;
}

/// Patient-specific repository operations
#[async_trait]
pub trait PatientRepository: Repository<Patient> {
    async fn find_by_identifier(&self, identifier: &str) -> Result<Option<Patient>, Self::Error>;
    async fn find_or_create_by_identifier(&self, patient_data: &Patient) -> Result<Patient, Self::Error>;
    async fn search_by_name(&self, last_name: &str, first_name: Option<&str>) -> Result<Vec<Patient>, Self::Error>;
    async fn find_by_birth_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Patient>, Self::Error>;
    async fn find_recent_patients(&self, days: u32) -> Result<Vec<Patient>, Self::Error>;
}

/// Test Result-specific repository operations
#[async_trait]
pub trait TestResultRepository: Repository<TestResult> {
    async fn find_by_patient_and_timerange(
        &self, 
        patient_id: &str, 
        start: DateTime<Utc>, 
        end: DateTime<Utc>
    ) -> Result<Vec<TestResult>, Self::Error>;
    
    async fn find_by_sample_id(&self, sample_id: &str) -> Result<Vec<TestResult>, Self::Error>;
    async fn find_by_test_id(&self, test_id: &str) -> Result<Vec<TestResult>, Self::Error>;
    async fn find_by_analyzer(&self, analyzer_id: &str) -> Result<Vec<TestResult>, Self::Error>;
    async fn find_by_status(&self, status: ResultStatus) -> Result<Vec<TestResult>, Self::Error>;
    async fn batch_insert(&self, results: &[TestResult]) -> Result<Vec<String>, Self::Error>;
    async fn find_abnormal_results(&self, limit: Option<usize>) -> Result<Vec<TestResult>, Self::Error>;
    async fn find_recent_results(&self, hours: u32) -> Result<Vec<TestResult>, Self::Error>;
}

/// Sample-specific repository operations
#[async_trait]
pub trait SampleRepository: Repository<Sample> {
    async fn find_by_status(&self, status: SampleStatus) -> Result<Vec<Sample>, Self::Error>;
    async fn find_by_sample_type(&self, sample_type: &str) -> Result<Vec<Sample>, Self::Error>;
    async fn find_by_position(&self, position: &str) -> Result<Option<Sample>, Self::Error>;
    async fn find_pending_samples(&self) -> Result<Vec<Sample>, Self::Error>;
    async fn find_samples_in_progress(&self) -> Result<Vec<Sample>, Self::Error>;
    async fn update_status(&self, id: &str, status: SampleStatus) -> Result<(), Self::Error>;
    async fn find_by_collection_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Sample>, Self::Error>;
}

/// Test Order-specific repository operations
#[async_trait]
pub trait TestOrderRepository: Repository<TestOrder> {
    async fn find_by_specimen_id(&self, specimen_id: &str) -> Result<Vec<TestOrder>, Self::Error>;
    async fn find_by_priority(&self, priority: &str) -> Result<Vec<TestOrder>, Self::Error>;
    async fn find_by_action_code(&self, action_code: &str) -> Result<Vec<TestOrder>, Self::Error>;
    async fn find_pending_orders(&self) -> Result<Vec<TestOrder>, Self::Error>;
    async fn find_by_ordering_provider(&self, provider: &str) -> Result<Vec<TestOrder>, Self::Error>;
    async fn find_by_date_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<TestOrder>, Self::Error>;
}

/// Upload Status-specific repository operations
#[async_trait]
pub trait UploadRepository: Repository<ResultUploadStatus> {
    async fn find_by_status(&self, status: UploadStatus) -> Result<Vec<ResultUploadStatus>, Self::Error>;
    async fn find_by_result_id(&self, result_id: &str) -> Result<Vec<ResultUploadStatus>, Self::Error>;
    async fn find_failed_uploads(&self) -> Result<Vec<ResultUploadStatus>, Self::Error>;
    async fn find_pending_uploads(&self) -> Result<Vec<ResultUploadStatus>, Self::Error>;
    async fn update_upload_status(&self, id: &str, status: UploadStatus, response_code: Option<String>, response_message: Option<String>) -> Result<(), Self::Error>;
    async fn increment_retry_count(&self, id: &str) -> Result<(), Self::Error>;
    async fn find_uploads_needing_retry(&self, max_retries: u32) -> Result<Vec<ResultUploadStatus>, Self::Error>;
}

/// System Settings repository for configuration management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSetting {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait SystemSettingsRepository: Repository<SystemSetting> {
    async fn get_setting(&self, key: &str) -> Result<Option<String>, Self::Error>;
    async fn set_setting(&self, key: &str, value: &str, description: Option<&str>, category: &str) -> Result<(), Self::Error>;
    async fn get_settings_by_category(&self, category: &str) -> Result<Vec<SystemSetting>, Self::Error>;
    async fn delete_setting(&self, key: &str) -> Result<(), Self::Error>;
    async fn get_all_settings(&self) -> Result<HashMap<String, String>, Self::Error>;
}

/// Machine Connector trait for analyzer communication
#[async_trait]
pub trait MachineConnector {
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn disconnect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn is_connected(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
    async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    async fn listen_for_data(&self) -> Result<MachineDataStream, Box<dyn std::error::Error + Send + Sync>>;
    fn machine_identifier(&self) -> &str;
    fn connection_info(&self) -> &str;
}

/// Stream for receiving machine data
pub struct MachineDataStream {
    pub data: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

/// Storage service that coordinates all repositories
#[async_trait]
pub trait StorageService {
    type Error: std::error::Error + Send + Sync;
    
    // TODO: Implement these repository accessors
    // fn analyzers(&self) -> &dyn AnalyzerRepository<Error = Self::Error, Id = String>;
    // fn patients(&self) -> &dyn PatientRepository<Error = Self::Error, Id = String>;
    // fn test_results(&self) -> &dyn TestResultRepository<Error = Self::Error, Id = String>;
    // fn samples(&self) -> &dyn SampleRepository<Error = Self::Error, Id = String>;
    // fn test_orders(&self) -> &dyn TestOrderRepository<Error = Self::Error, Id = String>;
    // fn uploads(&self) -> &dyn UploadRepository<Error = Self::Error, Id = String>;
    // fn system_settings(&self) -> &dyn SystemSettingsRepository<Error = Self::Error, Id = String>;
    
    async fn initialize(&self) -> Result<(), Self::Error>;
    // TODO: Implement these methods
    // async fn migrate(&self) -> Result<(), Self::Error>;
    // async fn backup(&self) -> Result<(), Self::Error>;
    // async fn restore(&self, backup_path: &str) -> Result<(), Self::Error>;
} 