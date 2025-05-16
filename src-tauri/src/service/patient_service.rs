use crate::model::patient::Patient;
use crate::storage::repository::sqlite::SqliteRepository;
use anyhow::Result;
use log::{debug, error, info, trace};
use std::sync::Arc;

/// Service for managing patient data
pub struct PatientService {
    repository: Arc<SqliteRepository>,
}

impl PatientService {
    /// Create a new patient service with the given repository
    pub fn new(repository: Arc<SqliteRepository>) -> Self {
        info!("Creating new PatientService");
        Self { repository }
    }

    /// Save a patient to the database
    pub async fn save_patient(&self, patient: &Patient) -> Result<i64> {
        info!("Saving patient with ID: {}", patient.id);
        trace!("Patient details: {:?}", patient);

        match self.repository.save_patient(patient).await {
            Ok(id) => {
                info!("Patient saved successfully with database ID: {}", id);
                Ok(id)
            }
            Err(e) => {
                error!("Failed to save patient {}: {}", patient.id, e);
                Err(e)
            }
        }
    }

    /// Get test results for a patient by ID
    pub async fn get_patient_results(
        &self,
        patient_id: i64,
    ) -> Result<Vec<crate::model::result::TestResult>> {
        info!("Getting test results for patient ID: {}", patient_id);

        match self.repository.get_patient_results(patient_id).await {
            Ok(results) => {
                info!(
                    "Retrieved {} test results for patient ID: {}",
                    results.len(),
                    patient_id
                );
                debug!(
                    "Result IDs: {:?}",
                    results.iter().map(|r| &r.id).collect::<Vec<_>>()
                );
                Ok(results)
            }
            Err(e) => {
                error!("Failed to get results for patient {}: {}", patient_id, e);
                Err(e)
            }
        }
    }

    /// Find a patient by ID
    pub async fn find_patient_by_id(&self, patient_id: &str) -> Result<Option<Patient>> {
        info!("Finding patient by ID: {}", patient_id);

        match self.repository.find_patient_by_id(patient_id).await {
            Ok(patient_opt) => {
                match &patient_opt {
                    Some(patient) => info!("Found patient with ID: {}", patient_id),
                    None => info!("No patient found with ID: {}", patient_id),
                }
                Ok(patient_opt)
            }
            Err(e) => {
                error!("Error finding patient with ID {}: {}", patient_id, e);
                Err(e)
            }
        }
    }

    /// Find patients by sample IDs
    pub async fn get_patients_by_sample_ids(&self, sample_ids: &[String]) -> Result<Vec<Patient>> {
        info!(
            "Getting patients by sample IDs, count: {}",
            sample_ids.len()
        );
        debug!("Sample IDs: {:?}", sample_ids);

        match self.repository.get_patients_by_sample_ids(sample_ids).await {
            Ok(patients) => {
                info!(
                    "Found {} patients for {} sample IDs",
                    patients.len(),
                    sample_ids.len()
                );
                Ok(patients)
            }
            Err(e) => {
                error!("Failed to get patients by sample IDs: {}", e);
                Err(e)
            }
        }
    }
}
