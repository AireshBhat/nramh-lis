use crate::model::result::{ResultStatus, TestResult, TestResultMetadata};
use crate::model::upload::UploadStatus;
use crate::storage::repository::sqlite::SqliteRepository;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error, info, trace, warn};
use std::sync::Arc;
use uuid::Uuid;

/// Service for managing test results
pub struct ResultService {
    repository: Arc<SqliteRepository>,
}

impl ResultService {
    /// Create a new result service with the given repository
    pub fn new(repository: Arc<SqliteRepository>) -> Self {
        info!("Creating new ResultService");
        Self { repository }
    }

    /// Save a test result to the database
    pub async fn save_result(&self, result: &TestResult) -> Result<i64> {
        info!("Saving test result with ID: {}", result.id);
        trace!(
            "Result details: Test ID: {}, Sample ID: {}, Value: {}",
            result.test_id,
            result.sample_id,
            result.value
        );

        match self.repository.save_test_result(result).await {
            Ok(id) => {
                info!("Test result saved successfully with database ID: {}", id);
                Ok(id)
            }
            Err(e) => {
                error!("Failed to save test result {}: {}", result.id, e);
                Err(e)
            }
        }
    }

    /// Create a new test result with the provided data
    pub async fn create_result(
        &self,
        test_id: &str,
        _patient_id: &str,
        sample_id: &str,
        value: &str,
        units: Option<String>,
        analyzer_id: &str,
    ) -> Result<TestResult> {
        // Generate a unique ID for the result
        let result_id = format!("R-{}", Uuid::new_v4());
        let now = Utc::now();

        info!(
            "Creating new test result: test_id={}, sample_id={}",
            test_id, sample_id
        );
        debug!(
            "Result details: value={}, units={:?}, analyzer={}",
            value, units, analyzer_id
        );

        // Create a basic test result
        let result = TestResult {
            id: result_id.clone(),
            test_id: test_id.to_string(),
            sample_id: sample_id.to_string(),
            value: value.to_string(),
            units,
            reference_range: None,       // To be added if available
            flags: None,                 // To be added if available
            status: ResultStatus::Final, // Default status
            completed_date_time: Some(now),
            metadata: TestResultMetadata {
                sequence_number: 1, // Default sequence
                instrument: Some(analyzer_id.to_string()),
            },
            analyzer_id: Some(analyzer_id.to_string()),
            created_at: now,
            updated_at: now,
        };

        // Save the result to the database
        let id = match self.save_result(&result).await {
            Ok(id) => id,
            Err(e) => {
                error!(
                    "Failed to save newly created test result {}: {}",
                    result_id, e
                );
                return Err(anyhow!("Failed to save test result"));
            }
        };

        if id <= 0 {
            error!(
                "Received invalid ID {} when saving test result {}",
                id, result_id
            );
            return Err(anyhow!("Failed to save test result"));
        }

        info!(
            "Successfully created and saved test result with ID: {}",
            result_id
        );
        Ok(result)
    }

    /// Track the upload of a result to an external system
    pub async fn track_result_upload(
        &self,
        result_id: &str,
        external_system_id: &str,
    ) -> Result<i64> {
        info!(
            "Tracking upload of result {} to external system {}",
            result_id, external_system_id
        );

        match self
            .repository
            .track_result_upload(result_id, external_system_id)
            .await
        {
            Ok(id) => {
                info!("Successfully tracked upload with ID: {}", id);
                Ok(id)
            }
            Err(e) => {
                error!("Failed to track upload for result {}: {}", result_id, e);
                Err(e)
            }
        }
    }

    /// Update the status of a result upload
    pub async fn update_upload_status(
        &self,
        upload_id: &str,
        status: UploadStatus,
        response_code: Option<&str>,
        response_message: Option<&str>,
    ) -> Result<bool> {
        info!(
            "Updating upload status for upload ID {}: status={:?}",
            upload_id, status
        );
        debug!(
            "Response: code={:?}, message={:?}",
            response_code, response_message
        );

        match self
            .repository
            .update_upload_status(upload_id, status, response_code, response_message)
            .await
        {
            Ok(success) => {
                if success {
                    info!("Successfully updated upload status for {}", upload_id);
                } else {
                    warn!(
                        "Update operation completed but status may not have been updated for {}",
                        upload_id
                    );
                }
                Ok(success)
            }
            Err(e) => {
                error!("Failed to update upload status for {}: {}", upload_id, e);
                Err(e)
            }
        }
    }

    /// Get pending result uploads
    pub async fn get_pending_uploads(
        &self,
        limit: u32,
    ) -> Result<Vec<crate::model::upload::ResultUploadStatus>> {
        info!("Getting pending uploads with limit: {}", limit);

        match self.repository.get_pending_uploads(limit).await {
            Ok(uploads) => {
                info!("Retrieved {} pending uploads", uploads.len());
                if !uploads.is_empty() {
                    debug!(
                        "Pending upload IDs: {:?}",
                        uploads.iter().map(|u| &u.id).collect::<Vec<_>>()
                    );
                }
                Ok(uploads)
            }
            Err(e) => {
                error!("Failed to get pending uploads: {}", e);
                Err(e)
            }
        }
    }

    /// Get upload status for a result
    pub async fn get_result_upload_status(
        &self,
        result_id: &str,
    ) -> Result<Vec<crate::model::upload::ResultUploadStatus>> {
        info!("Getting upload status for result ID: {}", result_id);

        match self.repository.get_result_upload_status(result_id).await {
            Ok(statuses) => {
                info!(
                    "Retrieved {} upload statuses for result {}",
                    statuses.len(),
                    result_id
                );
                Ok(statuses)
            }
            Err(e) => {
                error!(
                    "Failed to get upload status for result {}: {}",
                    result_id, e
                );
                Err(e)
            }
        }
    }
}
