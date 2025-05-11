use crate::model::analyzer::{Analyzer, AnalyzerStatus, ConnectionType};
use crate::model::patient::Patient;
use crate::model::result::{
    ReferenceRange, ResultFlags, ResultStatus, TestResult, TestResultMetadata,
};
use crate::model::upload::{ResultUploadStatus, UploadStatus};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use sqlx::{
    sqlite::SqlitePool,
    Row,
};
use uuid::Uuid;

use super::db;

/// SQLite repository for data access
pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    /// Create a new SQLite repository
    pub async fn new() -> Result<Self> {
        match db::establish_connection().await {
            Ok(pool) => {
                // Repository is initialized without schema creation
                // The schema will be managed by migrations
                Ok(Self { pool })
            }
            Err(e) => Err(anyhow!("Failed to establish connection: {}", e)),
        }
    }

    /// Save a patient to the database
    pub async fn save_patient(&self, patient: &Patient) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        // Check if patient exists
        let existing = sqlx::query("SELECT id FROM patients WHERE patient_id = ?")
            .bind(&patient.id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = existing {
            // Update existing patient
            let id: i64 = row.get("id");

            sqlx::query(
                "UPDATE patients SET 
                first_name = ?, 
                last_name = ?, 
                middle_name = ?,
                title = ?,
                birth_date = ?,
                sex = ?,
                street = ?,
                city = ?,
                state = ?,
                zip = ?,
                country_code = ?,
                telephone = ?,
                ordering_physician = ?,
                attending_physician = ?,
                referring_physician = ?,
                height_value = ?,
                height_unit = ?,
                weight_value = ?,
                weight_unit = ?,
                updated_at = ?
                WHERE patient_id = ?",
            )
            .bind(&patient.name.first_name)
            .bind(&patient.name.last_name)
            .bind(&patient.name.middle_name)
            .bind(&patient.name.title)
            .bind(patient.birth_date.map(|d| d.to_rfc3339()))
            .bind(&patient.sex.to_string())
            .bind(&patient.address.as_ref().and_then(|a| a.street.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.city.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.state.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.zip.clone()))
            .bind(
                &patient
                    .address
                    .as_ref()
                    .and_then(|a| a.country_code.clone()),
            )
            .bind(patient.telephone.join(","))
            .bind(&patient.physicians.as_ref().and_then(|p| p.ordering.clone()))
            .bind(
                &patient
                    .physicians
                    .as_ref()
                    .and_then(|p| p.attending.clone()),
            )
            .bind(
                &patient
                    .physicians
                    .as_ref()
                    .and_then(|p| p.referring.clone()),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.height.as_ref().map(|h| h.value)),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.height.as_ref().map(|h| h.unit.clone())),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.weight.as_ref().map(|w| w.value)),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.weight.as_ref().map(|w| w.unit.clone())),
            )
            .bind(&now)
            .bind(&patient.id)
            .execute(&self.pool)
            .await?;

            Ok(id)
        } else {
            // Insert new patient
            let id = sqlx::query(
                "INSERT INTO patients (
                    patient_id, first_name, last_name, middle_name, title, 
                    birth_date, sex, street, city, state, zip, country_code,
                    telephone, ordering_physician, attending_physician, referring_physician,
                    height_value, height_unit, weight_value, weight_unit, 
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&patient.id)
            .bind(&patient.name.first_name)
            .bind(&patient.name.last_name)
            .bind(&patient.name.middle_name)
            .bind(&patient.name.title)
            .bind(patient.birth_date.map(|d| d.to_rfc3339()))
            .bind(&patient.sex.to_string())
            .bind(&patient.address.as_ref().and_then(|a| a.street.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.city.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.state.clone()))
            .bind(&patient.address.as_ref().and_then(|a| a.zip.clone()))
            .bind(
                &patient
                    .address
                    .as_ref()
                    .and_then(|a| a.country_code.clone()),
            )
            .bind(patient.telephone.join(","))
            .bind(&patient.physicians.as_ref().and_then(|p| p.ordering.clone()))
            .bind(
                &patient
                    .physicians
                    .as_ref()
                    .and_then(|p| p.attending.clone()),
            )
            .bind(
                &patient
                    .physicians
                    .as_ref()
                    .and_then(|p| p.referring.clone()),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.height.as_ref().map(|h| h.value)),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.height.as_ref().map(|h| h.unit.clone())),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.weight.as_ref().map(|w| w.value)),
            )
            .bind(
                &patient
                    .physical_attributes
                    .as_ref()
                    .and_then(|p| p.weight.as_ref().map(|w| w.unit.clone())),
            )
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

            Ok(id)
        }
    }

    /// Save a test result to the database
    pub async fn save_test_result(&self, result: &TestResult) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        // Status conversion
        let status_str = result.status.to_string();

        // Extract reference range values if available
        let (lower_range, upper_range) = match &result.reference_range {
            Some(range) => (range.lower_limit, range.upper_limit),
            None => (None, None),
        };

        // Extract flags if available
        let (abnormal_flag, nature_of_abnormality) = match &result.flags {
            Some(flags) => (
                flags.abnormal_flag.clone(),
                flags.nature_of_abnormality.clone(),
            ),
            None => (None, None),
        };

        // Convert the TestResult to a format matching our migrations schema
        let id = sqlx::query(
            "INSERT INTO test_results (
                result_id, test_id, patient_id, sample_id, value, units,
                reference_range_lower, reference_range_upper, abnormal_flag,
                nature_of_abnormality, status, completed_date_time,
                sequence_number, instrument, analyzer_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&result.id)
        .bind(&result.test_id)
        .bind(&result.sample_id) // Note: This doesn't include patient ID, so you'll need to adapt your schema or add patient_id
        .bind(&result.value)
        .bind(&result.units)
        .bind(lower_range)
        .bind(upper_range)
        .bind(abnormal_flag)
        .bind(nature_of_abnormality)
        .bind(status_str)
        .bind(result.completed_date_time.map(|d| d.to_rfc3339()))
        .bind(result.metadata.sequence_number)
        .bind(&result.metadata.instrument)
        .bind(&result.analyzer_id)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    /// Get test results for a patient
    pub async fn get_patient_results(&self, patient_id: i64) -> Result<Vec<TestResult>> {
        let rows = sqlx::query(
            "SELECT tr.*, p.patient_id as pid 
             FROM test_results tr
             JOIN patients p ON tr.patient_id = p.patient_id
             WHERE p.id = ?
             ORDER BY tr.created_at DESC",
        )
        .bind(patient_id)
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();

        for row in rows {
            // Parse dates
            let completed_date_time = row
                .get::<Option<String>, _>("completed_date_time")
                .map(|s| s.parse::<DateTime<Utc>>().ok())
                .flatten();

            let created_at = row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse created_at: {}", e))?;

            let updated_at = row
                .get::<String, _>("updated_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse updated_at: {}", e))?;

            // Parse status
            let status = match row.get::<String, _>("status").as_str() {
                "C" => ResultStatus::Correction,
                "P" => ResultStatus::Preliminary,
                _ => ResultStatus::Final,
            };

            // Create reference range if values exist
            let reference_range = {
                let lower: Option<f64> = row.get("reference_range_lower");
                let upper: Option<f64> = row.get("reference_range_upper");

                if lower.is_some() || upper.is_some() {
                    Some(ReferenceRange {
                        lower_limit: lower,
                        upper_limit: upper,
                    })
                } else {
                    None
                }
            };

            // Create flags if values exist
            let flags = {
                let abnormal: Option<String> = row.get("abnormal_flag");
                let nature: Option<String> = row.get("nature_of_abnormality");

                if abnormal.is_some() || nature.is_some() {
                    Some(ResultFlags {
                        abnormal_flag: abnormal,
                        nature_of_abnormality: nature,
                    })
                } else {
                    None
                }
            };

            results.push(TestResult {
                id: row.get("result_id"),
                test_id: row.get("test_id"),
                sample_id: row.get("sample_id"),
                value: row.get("value"),
                units: row.get("units"),
                reference_range,
                flags,
                status,
                completed_date_time,
                metadata: TestResultMetadata {
                    sequence_number: row.get::<i32, _>("sequence_number") as u32,
                    instrument: row.get("instrument"),
                },
                analyzer_id: row.get("analyzer_id"),
                created_at,
                updated_at,
            });
        }

        Ok(results)
    }

    /// Save an analyzer to the database
    pub async fn save_analyzer(&self, analyzer: &Analyzer) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        // Check if analyzer exists
        let existing = sqlx::query("SELECT id FROM analyzers WHERE analyzer_id = ?")
            .bind(&analyzer.id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = existing {
            // Update existing analyzer
            let id: i64 = row.get("id");

            sqlx::query(
                "UPDATE analyzers SET 
                name = ?, 
                model = ?, 
                serial_number = ?,
                manufacturer = ?,
                connection_type = ?,
                ip_address = ?,
                port = ?,
                com_port = ?,
                baud_rate = ?,
                status = ?,
                updated_at = ?
                WHERE analyzer_id = ?",
            )
            .bind(&analyzer.name)
            .bind(&analyzer.model)
            .bind(&analyzer.serial_number)
            .bind(&analyzer.manufacturer)
            .bind(&analyzer.connection_type.to_string())
            .bind(&analyzer.ip_address)
            .bind(analyzer.port)
            .bind(&analyzer.com_port)
            .bind(analyzer.baud_rate)
            .bind(&analyzer.status.to_string())
            .bind(&now)
            .bind(&analyzer.id)
            .execute(&self.pool)
            .await?;

            Ok(id)
        } else {
            // Insert new analyzer
            let id = sqlx::query(
                "INSERT INTO analyzers (
                    analyzer_id, name, model, serial_number, manufacturer, 
                    connection_type, ip_address, port, com_port, baud_rate,
                    status, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&analyzer.id)
            .bind(&analyzer.name)
            .bind(&analyzer.model)
            .bind(&analyzer.serial_number)
            .bind(&analyzer.manufacturer)
            .bind(&analyzer.connection_type.to_string())
            .bind(&analyzer.ip_address)
            .bind(analyzer.port)
            .bind(&analyzer.com_port)
            .bind(analyzer.baud_rate)
            .bind(&analyzer.status.to_string())
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

            Ok(id)
        }
    }

    /// Get all analyzers
    pub async fn get_analyzers(&self) -> Result<Vec<Analyzer>> {
        let rows = sqlx::query("SELECT * FROM analyzers ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut analyzers = Vec::new();

        for row in rows {
            // Parse dates
            let created_at = row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse created_at: {}", e))?;

            let updated_at = row
                .get::<String, _>("updated_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse updated_at: {}", e))?;

            // Parse connection_type
            let connection_type = ConnectionType::from(row.get::<String, _>("connection_type").as_str());

            // Parse status
            let status = AnalyzerStatus::from(row.get::<String, _>("status").as_str());
            
            analyzers.push(Analyzer {
                id: row.get("analyzer_id"),
                name: row.get("name"),
                model: row.get("model"),
                serial_number: row.get("serial_number"),
                manufacturer: row.get("manufacturer"),
                connection_type,
                ip_address: row.get("ip_address"),
                port: row.get::<Option<i32>, _>("port").map(|p| p as u16),
                com_port: row.get("com_port"),
                baud_rate: row.get::<Option<i32>, _>("baud_rate").map(|b| b as u32),
                status,
                created_at,
                updated_at,
            });
        }

        Ok(analyzers)
    }

    /// Get an analyzer by ID
    pub async fn get_analyzer(&self, analyzer_id: &str) -> Result<Option<Analyzer>> {
        let row = sqlx::query("SELECT * FROM analyzers WHERE analyzer_id = ?")
            .bind(analyzer_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            // Parse dates
            let created_at = row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse created_at: {}", e))?;

            let updated_at = row
                .get::<String, _>("updated_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse updated_at: {}", e))?;

            // Parse connection_type
            let connection_type = ConnectionType::from(row.get::<String, _>("connection_type").as_str());

            // Parse status
            let status = AnalyzerStatus::from(row.get::<String, _>("status").as_str());
            
            Ok(Some(Analyzer {
                id: row.get("analyzer_id"),
                name: row.get("name"),
                model: row.get("model"),
                serial_number: row.get("serial_number"),
                manufacturer: row.get("manufacturer"),
                connection_type,
                ip_address: row.get("ip_address"),
                port: row.get::<Option<i32>, _>("port").map(|p| p as u16),
                com_port: row.get("com_port"),
                baud_rate: row.get::<Option<i32>, _>("baud_rate").map(|b| b as u32),
                status,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Delete an analyzer
    pub async fn delete_analyzer(&self, analyzer_id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM analyzers WHERE analyzer_id = ?")
            .bind(analyzer_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Track result upload
    pub async fn track_result_upload(
        &self,
        result_id: &str,
        external_system_id: &str,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        let upload_id = Uuid::new_v4().to_string();

        let id = sqlx::query(
            "INSERT INTO result_upload_status (
                upload_id, result_id, external_system_id, upload_status,
                retry_count, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&upload_id)
        .bind(result_id)
        .bind(external_system_id)
        .bind("PENDING")
        .bind(0)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    /// Update result upload status
    pub async fn update_upload_status(
        &self,
        upload_id: &str,
        status: UploadStatus,
        response_code: Option<&str>,
        response_message: Option<&str>,
    ) -> Result<bool> {
        let now = Utc::now().to_rfc3339();
        let upload_date = if matches!(status, UploadStatus::Uploaded) {
            Some(now.clone())
        } else {
            None
        };

        let query = match status {
            UploadStatus::Uploaded => {
                "UPDATE result_upload_status SET 
                upload_status = ?, 
                upload_date = ?,
                response_code = ?,
                response_message = ?,
                updated_at = ?
                WHERE upload_id = ?"
            }
            UploadStatus::Failed => {
                "UPDATE result_upload_status SET 
                upload_status = ?, 
                response_code = ?,
                response_message = ?,
                retry_count = retry_count + 1,
                updated_at = ?
                WHERE upload_id = ?"
            }
            _ => {
                "UPDATE result_upload_status SET 
                upload_status = ?, 
                updated_at = ?
                WHERE upload_id = ?"
            }
        };

        let result = match status {
            UploadStatus::Uploaded => {
                sqlx::query(query)
                    .bind(status.to_string())
                    .bind(upload_date)
                    .bind(response_code)
                    .bind(response_message)
                    .bind(&now)
                    .bind(upload_id)
                    .execute(&self.pool)
                    .await?
            }
            UploadStatus::Failed => {
                sqlx::query(query)
                    .bind(status.to_string())
                    .bind(response_code)
                    .bind(response_message)
                    .bind(&now)
                    .bind(upload_id)
                    .execute(&self.pool)
                    .await?
            }
            _ => {
                sqlx::query(query)
                    .bind(status.to_string())
                    .bind(&now)
                    .bind(upload_id)
                    .execute(&self.pool)
                    .await?
            }
        };

        Ok(result.rows_affected() > 0)
    }

    /// Get pending result uploads
    pub async fn get_pending_uploads(&self, limit: u32) -> Result<Vec<ResultUploadStatus>> {
        let rows = sqlx::query(
            "SELECT * FROM result_upload_status 
             WHERE upload_status = 'PENDING' 
             ORDER BY created_at ASC 
             LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut uploads = Vec::new();

        for row in rows {
            // Parse dates
            let created_at = row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse created_at: {}", e))?;

            let updated_at = row
                .get::<String, _>("updated_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse updated_at: {}", e))?;

            let upload_date = row
                .get::<Option<String>, _>("upload_date")
                .map(|s| s.parse::<DateTime<Utc>>().ok())
                .flatten();

            // Parse status
            let status = UploadStatus::from(row.get::<String, _>("upload_status").as_str());

            uploads.push(ResultUploadStatus {
                id: row.get("upload_id"),
                result_id: row.get("result_id"),
                external_system_id: row.get("external_system_id"),
                status,
                upload_date,
                response_code: row.get("response_code"),
                response_message: row.get("response_message"),
                retry_count: row.get::<i32, _>("retry_count") as u32,
                created_at,
                updated_at,
            });
        }

        Ok(uploads)
    }

    /// Get upload status for a result
    pub async fn get_result_upload_status(&self, result_id: &str) -> Result<Vec<ResultUploadStatus>> {
        let rows = sqlx::query(
            "SELECT * FROM result_upload_status 
             WHERE result_id = ? 
             ORDER BY created_at DESC",
        )
        .bind(result_id)
        .fetch_all(&self.pool)
        .await?;

        let mut uploads = Vec::new();

        for row in rows {
            // Parse dates
            let created_at = row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse created_at: {}", e))?;

            let updated_at = row
                .get::<String, _>("updated_at")
                .parse::<DateTime<Utc>>()
                .map_err(|e| anyhow!("Failed to parse updated_at: {}", e))?;

            let upload_date = row
                .get::<Option<String>, _>("upload_date")
                .map(|s| s.parse::<DateTime<Utc>>().ok())
                .flatten();

            // Parse status
            let status = UploadStatus::from(row.get::<String, _>("upload_status").as_str());

            uploads.push(ResultUploadStatus {
                id: row.get("upload_id"),
                result_id: row.get("result_id"),
                external_system_id: row.get("external_system_id"),
                status,
                upload_date,
                response_code: row.get("response_code"),
                response_message: row.get("response_message"),
                retry_count: row.get::<i32, _>("retry_count") as u32,
                created_at,
                updated_at,
            });
        }

        Ok(uploads)
    }
}
