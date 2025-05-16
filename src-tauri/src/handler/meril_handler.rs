use crate::{
    service::{MerilMachineService, PatientService, ResultService},
    storage::SqliteRepository,
};
use anyhow::Result;
use log::{debug, error, info, warn};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

/// Handler for Meril analyzer communication
pub struct MerilHandler {
    patient_service: Arc<PatientService>,
    result_service: Arc<ResultService>,
    machine_service: MerilMachineService,
    event_tx: Sender<String>,
}

impl MerilHandler {
    /// Create a new MerilHandler with the given services
    pub fn new(repository: Arc<SqliteRepository>, event_tx: Sender<String>, port: u16) -> Self {
        info!("Creating new MerilHandler with port {}", port);

        let patient_service = Arc::new(PatientService::new(repository.clone()));
        let result_service = Arc::new(ResultService::new(repository.clone()));

        debug!("Creating MerilMachineService with port {}", port);
        // Create the machine service
        let machine_service = MerilMachineService::new(
            Arc::clone(&patient_service),
            Arc::clone(&result_service),
            port,
        );

        info!("MerilHandler initialized successfully");
        Self {
            patient_service,
            result_service,
            machine_service,
            event_tx,
        }
    }

    /// Start the Meril machine service
    pub async fn start_service(&self) -> Result<()> {
        info!("Starting Meril machine service");

        // Start the TCP server for the machine service
        match self.machine_service.start_server().await {
            Ok(_) => {
                info!("Meril machine service started successfully");

                // Emit an event to notify the UI
                let event = format!(
                    "Meril machine service started on port {}",
                    self.machine_service.port()
                );

                if let Err(e) = self.event_tx.send(event).await {
                    warn!("Failed to send event notification: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to start Meril machine service: {}", e);
                Err(e)
            }
        }
    }

    /// Stop the Meril machine service
    pub fn stop_service(&self) -> Result<()> {
        info!("Stopping Meril machine service");

        // Stop the TCP server
        match self.machine_service.stop_server() {
            Ok(_) => {
                info!("Meril machine service stopped successfully");

                // Emit an event to notify the UI
                let event = "Meril machine service stopped".to_string();
                if let Err(e) = self.event_tx.try_send(event) {
                    warn!("Failed to send stop event notification: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to stop Meril machine service: {}", e);
                Err(e)
            }
        }
    }

    /// Handle a new test result from the analyzer
    pub async fn handle_test_result(
        &self,
        test_id: &str,
        patient_id: &str,
        sample_id: &str,
        value: &str,
        units: Option<String>,
        analyzer_id: &str,
    ) -> Result<()> {
        info!(
            "Handling new test result: test_id={}, patient_id={}, sample_id={}",
            test_id, patient_id, sample_id
        );
        debug!(
            "Result data: value={}, units={:?}, analyzer={}",
            value, units, analyzer_id
        );

        // Create the test result
        match self
            .result_service
            .create_result(test_id, patient_id, sample_id, value, units, analyzer_id)
            .await
        {
            Ok(result) => {
                info!("Successfully created result with ID: {}", result.id);

                // Emit an event to notify the UI
                let event = format!("New result received: {} - {}", test_id, value);
                if let Err(e) = self.event_tx.send(event).await {
                    warn!("Failed to send result event notification: {}", e);
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to create test result: {}", e);
                Err(e)
            }
        }
    }

    /// Process patient data from the analyzer
    pub async fn process_patient_data(
        &self,
        patient: &crate::model::patient::Patient,
    ) -> Result<i64> {
        info!("Processing patient data for patient ID: {}", patient.id);

        // Save the patient
        match self.patient_service.save_patient(patient).await {
            Ok(id) => {
                info!(
                    "Successfully processed and saved patient with database ID: {}",
                    id
                );
                Ok(id)
            }
            Err(e) => {
                error!("Failed to process patient data: {}", e);
                Err(e)
            }
        }
    }

    /// Upload a result to an external system
    pub async fn upload_result(&self, result_id: &str, external_system_id: &str) -> Result<i64> {
        info!(
            "Uploading result {} to external system {}",
            result_id, external_system_id
        );

        // Track the upload
        match self
            .result_service
            .track_result_upload(result_id, external_system_id)
            .await
        {
            Ok(upload_id) => {
                info!("Result queued for upload with tracking ID: {}", upload_id);

                // Emit an event to notify the UI
                let event = format!(
                    "Result {} queued for upload to {}",
                    result_id, external_system_id
                );
                if let Err(e) = self.event_tx.send(event).await {
                    warn!("Failed to send upload event notification: {}", e);
                }

                Ok(upload_id)
            }
            Err(e) => {
                error!("Failed to upload result: {}", e);
                Err(e)
            }
        }
    }

    /// Check if the service is running
    pub fn is_service_running(&self) -> bool {
        let running = self.machine_service.is_running();
        debug!("Checking if Meril service is running: {}", running);
        running
    }
}
