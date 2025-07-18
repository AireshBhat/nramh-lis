use std::sync::Arc;
use tauri::{AppHandle, Emitter, Runtime};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::models::{ Analyzer, hematology::BF6900Event };
use crate::services::autoquant_meril::AutoQuantMerilService;
use crate::services::bf6900_service::BF6900Service;
use crate::services::his_client::HisClient;

/// Central application state manager
pub struct AppState<R: Runtime> {
    autoquant_meril_service: Arc<AutoQuantMerilService<R>>,
    bf6900_service: Arc<BF6900Service<R>>,
    his_client: Arc<HisClient>,
    meril_service_handle: Option<JoinHandle<Result<(), String>>>,
    bf6900_service_handle: Option<JoinHandle<Result<(), String>>>,
}

impl<R: Runtime> AppState<R> {
    /// Creates a new AppState instance
    pub fn new(
        app_handle: AppHandle<R>,
        meril_store: Arc<tauri_plugin_store::Store<R>>,
        bf6900_store: Arc<tauri_plugin_store::Store<R>>,
    ) -> Result<Self, String> {
        // Create event channel for AutoQuantMeril service
        let (event_sender, event_receiver) =
            mpsc::channel::<crate::services::autoquant_meril::MerilEvent>(100);

        // Get analyzer configuration from store
        let config_value = meril_store.get("config");
        let analyzer = if let Some(value) = config_value {
            // Try to deserialize the stored value
            let store_data: Result<crate::api::commands::meril_handler::MerilStoreData, _> =
                serde_json::from_value(value.clone());

            match store_data {
                Ok(data) => {
                    if let Some(analyzer) = data.analyzer {
                        analyzer
                    } else {
                        // Create default analyzer if none exists
                        Self::create_default_meril_analyzer()
                    }
                }
                Err(_) => {
                    // Invalid JSON, create default analyzer
                    Self::create_default_meril_analyzer()
                }
            }
        } else {
            // No config, create default analyzer
            Self::create_default_meril_analyzer()
        };

        // Create the AutoQuantMeril service
        let service = Arc::new(AutoQuantMerilService::<R>::new(
            analyzer,
            event_sender,
            meril_store,
        ));

        // Create HIS client
        let his_client = Arc::new(HisClient::with_default_config());

        // Start event handler for frontend communication
        let app_handle_clone = app_handle.clone();
        let his_client_clone = his_client.clone();
        tokio::spawn(async move {
            Self::handle_meril_events(app_handle_clone, event_receiver, his_client_clone).await;
        });

        // Create event channel for BF-6900 service
        let (bf6900_event_sender, bf6900_event_receiver) =
            mpsc::channel::<crate::models::hematology::BF6900Event>(100);

        // Get BF-6900 analyzer configuration from store
        let bf6900_config_value = bf6900_store.get("config");
        let bf6900_analyzer = if let Some(value) = bf6900_config_value {
            // Try to deserialize the stored value
            let store_data: Result<crate::api::commands::bf6900_handler::BF6900StoreData, _> =
                serde_json::from_value(value.clone());

            match store_data {
                Ok(data) => {
                    if let Some(analyzer) = data.analyzer {
                        analyzer
                    } else {
                        // Create default analyzer if none exists
                        Self::create_default_bf6900_analyzer()
                    }
                }
                Err(_) => {
                    // Invalid JSON, create default analyzer
                    Self::create_default_bf6900_analyzer()
                }
            }
        } else {
            // No config, create default analyzer
            Self::create_default_bf6900_analyzer()
        };

        // Create the BF-6900 service
        let bf6900_service = Arc::new(BF6900Service::<R>::new(
            bf6900_analyzer,
            bf6900_event_sender,
            bf6900_store,
        ));

        // Start event handler for BF-6900 frontend communication
        let app_handle_clone = app_handle.clone();
        let his_client_clone = his_client.clone();
        tokio::spawn(async move {
            Self::handle_bf6900_events(app_handle_clone, bf6900_event_receiver, his_client_clone).await;
        });

        let app_state = Self {
            autoquant_meril_service: service,
            bf6900_service,
            his_client,
            meril_service_handle: None,
            bf6900_service_handle: None,
        };

        Ok(app_state)
    }

    /// Initializes the AppState (called after creation to handle async operations)
    pub async fn initialize(&mut self) -> Result<(), String> {
        // Auto-start Meril service if configured
        let analyzer_config = self.autoquant_meril_service.get_analyzer_config().await;
        if analyzer_config.activate_on_start {
            log::info!("Auto-starting Meril service due to activate_on_start=true");
            self.start_meril_service_internal().await?;
        }

        // Auto-start BF-6900 service if configured
        let bf6900_config = self.bf6900_service.get_analyzer_config().await;
        if bf6900_config.activate_on_start {
            log::info!("Auto-starting BF-6900 service due to activate_on_start=true");
            self.start_bf6900_service_internal().await?;
        }

        Ok(())
    }

    /// Gets a reference to the AutoQuantMeril service
    pub fn get_autoquant_meril_service(&self) -> &Arc<AutoQuantMerilService<R>> {
        &self.autoquant_meril_service
    }

    /// Gets a reference to the BF-6900 service
    pub fn get_bf6900_service(&self) -> &Arc<BF6900Service<R>> {
        &self.bf6900_service
    }

    /// Starts the Meril service in a background thread
    pub async fn start_meril_service_internal(&mut self) -> Result<(), String> {
        // Check if service is already running
        if self.meril_service_handle.is_some() {
            return Err("Service is already running".to_string());
        }

        // Clone the service for the background thread
        let service = self.autoquant_meril_service.clone();

        // Spawn the service in a background thread
        let handle = tokio::spawn(async move { service.start().await });

        self.meril_service_handle = Some(handle);

        log::info!("Meril service started successfully");
        Ok(())
    }

    /// Stops the Meril service and waits for thread completion
    pub async fn stop_meril_service_internal(&mut self) -> Result<(), String> {
        // Check if service is running
        let handle = match &mut self.meril_service_handle {
            Some(h) => h,
            None => return Err("Service is not running".to_string()),
        };

        // Stop the service
        let service = self.autoquant_meril_service.clone();
        if let Err(e) = service.stop().await {
            log::error!("Error stopping service: {}", e);
        }

        // Wait for thread completion
        match handle.await {
            Ok(Ok(())) => {
                log::info!("Meril service stopped successfully");
                self.meril_service_handle = None;
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("Service thread returned error: {}", e);
                self.meril_service_handle = None;
                Err(e)
            }
            Err(e) => {
                log::error!("Failed to join service thread: {}", e);
                self.meril_service_handle = None;
                Err(format!("Thread join error: {}", e))
            }
        }
    }

    /// Gets the service status
    pub async fn get_service_status(&self) -> (bool, usize) {
        let is_running = self.meril_service_handle.is_some();
        let connections_count = self.autoquant_meril_service.get_connections_count().await;
        (is_running, connections_count)
    }

    /// Creates a default MERIL analyzer configuration
    pub fn create_default_meril_analyzer() -> Analyzer {
        use chrono::Utc;
        use uuid::Uuid;

        Analyzer {
            id: Uuid::new_v4().to_string(),
            name: "AutoQuant".to_string(),
            model: "200i".to_string(),
            serial_number: None,
            manufacturer: Some("Meril Diagnostics PVT LTD".to_string()),
            connection_type: crate::models::ConnectionType::TcpIp,
            ip_address: None,
            port: Some(5600), // Default port
            com_port: None,
            baud_rate: None,
            protocol: crate::models::Protocol::Astm,
            status: crate::models::AnalyzerStatus::Inactive,
            activate_on_start: true, // Don't auto-start by default
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Handles MERIL events and sends them to the frontend
    async fn handle_meril_events(
        app: AppHandle<R>,
        mut event_receiver: mpsc::Receiver<crate::services::autoquant_meril::MerilEvent>,
        his_client: Arc<HisClient>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                crate::services::autoquant_meril::MerilEvent::AnalyzerConnected {
                    analyzer_id,
                    remote_addr,
                    timestamp,
                } => {
                    log::info!("Analyzer {} connected from {}", analyzer_id, remote_addr);

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:analyzer-connected",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "remote_addr": remote_addr,
                            "timestamp": timestamp
                        }),
                    );
                }
                crate::services::autoquant_meril::MerilEvent::AnalyzerDisconnected {
                    analyzer_id,
                    timestamp,
                } => {
                    log::info!("Analyzer {} disconnected", analyzer_id);

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:analyzer-disconnected",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "timestamp": timestamp
                        }),
                    );
                }
                crate::services::autoquant_meril::MerilEvent::AstmMessageReceived {
                    analyzer_id,
                    message_type,
                    raw_data,
                    timestamp,
                } => {
                    log::debug!(
                        "ASTM message from {}: {} - {}",
                        analyzer_id,
                        message_type,
                        raw_data
                    );

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:astm-message",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "message_type": message_type,
                            "raw_data": raw_data,
                            "timestamp": timestamp
                        }),
                    );
                }
                crate::services::autoquant_meril::MerilEvent::LabResultProcessed {
                    analyzer_id,
                    patient_id,
                    patient_data,
                    test_results,
                    timestamp,
                } => {
                    log::info!(
                        "Lab results processed for analyzer {}: {} tests",
                        analyzer_id,
                        test_results.len()
                    );

                    // Send results to HIS system
                    if !test_results.is_empty() {
                        let his_client_clone = his_client.clone();
                        let analyzer_id_clone = analyzer_id.clone();
                        let patient_id_clone = patient_id.clone();
                        let test_results_clone = test_results.clone();
                        let timestamp_clone = timestamp;
                        
                        tokio::spawn(async move {
                            if let Err(e) = his_client_clone.send_meril_results(
                                &analyzer_id_clone,
                                patient_id_clone.as_deref(),
                                &test_results_clone,
                            ).await {
                                log::error!("Failed to send lab results to HIS system: {}", e);
                            } else {
                                log::info!("Successfully sent lab results to HIS system for analyzer {}", analyzer_id_clone);
                            }
                        });
                    }

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:lab-results",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "patient_id": patient_id,
                            "patient_data": patient_data,
                            "test_results": test_results,
                            "timestamp": timestamp
                        }),
                    );
                }
                crate::services::autoquant_meril::MerilEvent::AnalyzerStatusUpdated {
                    analyzer_id,
                    status,
                    timestamp,
                } => {
                    log::info!("Analyzer {} status updated to {:?}", analyzer_id, status);

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:analyzer-status-updated",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "status": status,
                            "timestamp": timestamp
                        }),
                    );
                }
                crate::services::autoquant_meril::MerilEvent::Error {
                    analyzer_id,
                    error,
                    timestamp,
                } => {
                    log::error!("Error in analyzer {}: {}", analyzer_id, error);

                    // Emit event to frontend
                    let _ = app.emit(
                        "meril:error",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "error": error,
                            "timestamp": timestamp
                        }),
                    );
                }
            }
        }
    }

    /// Starts the BF-6900 service in a background thread
    pub async fn start_bf6900_service_internal(&mut self) -> Result<(), String> {
        // Check if service is already running
        if self.bf6900_service_handle.is_some() {
            return Err("BF-6900 service is already running".to_string());
        }

        // Clone the service for the background thread
        let service = self.bf6900_service.clone();

        // Spawn the service in a background thread
        let handle = tokio::spawn(async move { service.start().await });

        self.bf6900_service_handle = Some(handle);

        log::info!("BF-6900 service started successfully");
        Ok(())
    }

    /// Stops the BF-6900 service and waits for thread completion
    pub async fn stop_bf6900_service_internal(&mut self) -> Result<(), String> {
        // Check if service is running
        let handle = match &mut self.bf6900_service_handle {
            Some(h) => h,
            None => return Err("BF-6900 service is not running".to_string()),
        };

        // Stop the service
        let service = self.bf6900_service.clone();
        if let Err(e) = service.stop().await {
            log::error!("Error stopping BF-6900 service: {}", e);
        }

        // Wait for thread completion
        match handle.await {
            Ok(Ok(())) => {
                log::info!("BF-6900 service stopped successfully");
                self.bf6900_service_handle = None;
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("BF-6900 service thread returned error: {}", e);
                self.bf6900_service_handle = None;
                Err(e)
            }
            Err(e) => {
                log::error!("Failed to join BF-6900 service thread: {}", e);
                self.bf6900_service_handle = None;
                Err(format!("Thread join error: {}", e))
            }
        }
    }

    /// Gets the BF-6900 service status
    pub async fn get_bf6900_service_status(&self) -> (bool, usize) {
        let is_running = self.bf6900_service_handle.is_some();
        let connections_count = self.bf6900_service.get_connections_count().await;
        (is_running, connections_count)
    }

    /// Creates a default BF-6900 analyzer configuration
    pub fn create_default_bf6900_analyzer() -> Analyzer {
        use chrono::Utc;
        use uuid::Uuid;

        Analyzer {
            id: Uuid::new_v4().to_string(),
            name: "Meril CQ 5 Plus".to_string(),
            model: "BF-6900".to_string(),
            serial_number: None,
            manufacturer: Some("Meril Diagnostics PVT LTD".to_string()),
            connection_type: crate::models::ConnectionType::TcpIp,
            ip_address: None,
            port: Some(9100), // Standard HL7 port
            com_port: None,
            baud_rate: None,
            protocol: crate::models::Protocol::Hl7V231,
            status: crate::models::AnalyzerStatus::Inactive,
            activate_on_start: true, // Don't auto-start by default
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Handles BF-6900 events and sends them to the frontend
    async fn handle_bf6900_events(
        app: AppHandle<R>,
        mut event_receiver: mpsc::Receiver<crate::models::hematology::BF6900Event>,
        his_client: Arc<HisClient>,
    ) {
        while let Some(event) = event_receiver.recv().await {
            match event {
                BF6900Event::AnalyzerConnected {
                    analyzer_id,
                    remote_addr,
                    timestamp,
                } => {
                    log::info!("BF-6900 Analyzer {} connected from {}", analyzer_id, remote_addr);

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:analyzer-connected",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "remote_addr": remote_addr,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::AnalyzerDisconnected {
                    analyzer_id,
                    timestamp,
                } => {
                    log::info!("BF-6900 Analyzer {} disconnected", analyzer_id);

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:analyzer-disconnected",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::HL7MessageReceived {
                    analyzer_id,
                    message_type,
                    raw_data,
                    timestamp,
                } => {
                    log::debug!(
                        "HL7 message from {}: {} - {}",
                        analyzer_id,
                        message_type,
                        raw_data
                    );

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:hl7-message",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "message_type": message_type,
                            "raw_data": raw_data,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::HematologyResultProcessed {
                    analyzer_id,
                    patient_id,
                    patient_data,
                    test_results,
                    timestamp,
                } => {
                    log::info!(
                        "BF-6900 hematology results processed for analyzer {}: {} tests",
                        analyzer_id,
                        test_results.len()
                    );

                    // Send results to HIS system
                    if !test_results.is_empty() {
                        let his_client_clone = his_client.clone();
                        let analyzer_id_clone = analyzer_id.clone();
                        let patient_id_clone = patient_id.clone();
                        let test_results_clone = test_results.clone();
                        let timestamp_clone = timestamp;
                        
                        tokio::spawn(async move {
                            if let Err(e) = his_client_clone.send_hematology_results(
                                &analyzer_id_clone,
                                patient_id_clone.as_deref(),
                                &test_results_clone,
                                timestamp_clone,
                            ).await {
                                log::error!("Failed to send hematology results to HIS system: {}", e);
                            } else {
                                log::info!("Successfully sent hematology results to HIS system for analyzer {}", analyzer_id_clone);
                            }
                        });
                    }

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:lab-results",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "patient_id": patient_id,
                            "patient_data": patient_data,
                            "test_results": test_results,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::AnalyzerStatusUpdated {
                    analyzer_id,
                    status,
                    timestamp,
                } => {
                    log::info!("BF-6900 Analyzer {} status updated to {:?}", analyzer_id, status);

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:analyzer-status-updated",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "status": status,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::CelquantIdentificationReceived {
                    analyzer_id,
                    device_name,
                    version,
                    message,
                    timestamp,
                } => {
                    log::info!("Celquant identification received for analyzer {}: {} - {}", analyzer_id, device_name, version);

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:celquant-identification",
                        serde_json::json!({
                            "analyzer_id": analyzer_id, 
                            "device_name": device_name,
                            "version": version,
                            "message": message,
                            "timestamp": timestamp
                        }),
                    );
                }
                BF6900Event::Error {
                    analyzer_id,
                    error,
                    timestamp,
                } => {
                    log::error!("Error in BF-6900 analyzer {}: {}", analyzer_id, error);

                    // Emit event to frontend
                    let _ = app.emit(
                        "bf6900:error",
                        serde_json::json!({
                            "analyzer_id": analyzer_id,
                            "error": error,
                            "timestamp": timestamp
                        }),
                    );
                }
            }
        }
    }
}
