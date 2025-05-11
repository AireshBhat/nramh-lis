use crate::model::patient::Patient;
use crate::model::result::{
    ReferenceRange, ResultFlags, ResultStatus, TestResult, TestResultMetadata,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePool},
    Row,
};

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
        let status_str = match result.status {
            ResultStatus::Correction => "C",
            ResultStatus::Final => "F",
            ResultStatus::Preliminary => "P",
        };

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
                sequence_number, instrument, is_uploaded, created_at, updated_at
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
        .bind(false) // is_uploaded
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
                created_at,
                updated_at,
            });
        }

        Ok(results)
    }

    /// Mark test results as uploaded
    pub async fn mark_results_as_uploaded(&self, result_ids: &[String]) -> Result<u64> {
        let now = Utc::now().to_rfc3339();

        // Building dynamic query for multiple result IDs
        let placeholders = (0..result_ids.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let query = format!(
            "UPDATE test_results SET is_uploaded = true, updated_at = ? WHERE result_id IN ({})",
            placeholders
        );

        // Start building the query
        let mut query_builder = sqlx::query(&query).bind(&now);

        // Add each result ID as a parameter
        for id in result_ids {
            query_builder = query_builder.bind(id);
        }

        // Execute and get affected rows
        let result = query_builder.execute(&self.pool).await?;

        Ok(result.rows_affected())
    }

    /// Get pending results that need to be uploaded
    pub async fn get_pending_uploads(&self, limit: u32) -> Result<Vec<TestResult>> {
        let rows = sqlx::query(
            "SELECT tr.*, p.patient_id as pid 
             FROM test_results tr
             JOIN patients p ON tr.patient_id = p.patient_id  
             WHERE tr.is_uploaded = false 
             ORDER BY tr.created_at ASC 
             LIMIT ?",
        )
        .bind(limit)
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
                created_at,
                updated_at,
            });
        }

        Ok(results)
    }
}
