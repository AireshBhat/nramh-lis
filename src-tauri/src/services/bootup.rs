use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;
use tokio::sync::mpsc;

use crate::services::autoquant_meril::{AutoQuantMerilService, MerilEvent};

pub async fn setup<R: tauri::Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let meril_store = app
        .store("meril.json")
        .map_err(|e| format!("Error getting store: {}", e))?;

    // Initialize AutoQuantMeril service
    initialize_autoquant_meril_service::<R>(app.clone(), meril_store).await?;

    // let _afinion_store = app
    //     .store("afinion.json")
    //     .map_err(|e| format!("Error getting store: {}", e))?;

    // let _bf6500_store = app
    //     .store("bf6500.json")
    //     .map_err(|e| format!("Error getting store: {}", e))?;

    log::info!("Bootup service initialized");
    Ok(())
}

/// Initializes the AutoQuantMeril service
async fn initialize_autoquant_meril_service<R: tauri::Runtime>(
    app: AppHandle<R>,
    meril_store: Arc<tauri_plugin_store::Store<R>>,
) -> Result<(), String> {
    // Create event channel for frontend communication
    let (event_sender, mut event_receiver) = mpsc::channel::<MerilEvent>(100);
    
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
                    create_default_meril_analyzer()
                }
            }
            Err(_) => {
                // Invalid JSON, create default analyzer
                create_default_meril_analyzer()
            }
        }
    } else {
        // No config, create default analyzer
        create_default_meril_analyzer()
    };
    
    // Create the service
    let mut service = AutoQuantMerilService::<R>::new(analyzer, event_sender, meril_store.clone());
    
    // Start the service
    // service.start().await?;
    
    // Store service in AppData for global access
    app.manage(Arc::new(service));
    
    // Start event handler for frontend communication
    tokio::spawn(async move {
        handle_meril_events::<R>(app, event_receiver).await;
    });
    
    log::info!("AutoQuantMeril service initialized successfully");
    Ok(())
}

/// Creates a default MERIL analyzer configuration
fn create_default_meril_analyzer() -> crate::models::Analyzer {
    use chrono::Utc;
    use uuid::Uuid;
    
    crate::models::Analyzer {
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
        activate_on_start: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Handles MERIL events and sends them to the frontend
async fn handle_meril_events<R: tauri::Runtime>(
    app: AppHandle<R>,
    mut event_receiver: mpsc::Receiver<MerilEvent>,
) {
    while let Some(event) = event_receiver.recv().await {
        match event {
            MerilEvent::AnalyzerConnected { analyzer_id, remote_addr, timestamp } => {
                log::info!("Analyzer {} connected from {}", analyzer_id, remote_addr);
                
                // Emit event to frontend
                let _ = app.emit("meril:analyzer-connected", serde_json::json!({
                    "analyzer_id": analyzer_id,
                    "remote_addr": remote_addr,
                    "timestamp": timestamp
                }));
            }
            MerilEvent::AnalyzerDisconnected { analyzer_id, timestamp } => {
                log::info!("Analyzer {} disconnected", analyzer_id);
                
                // Emit event to frontend
                let _ = app.emit("meril:analyzer-disconnected", serde_json::json!({
                    "analyzer_id": analyzer_id,
                    "timestamp": timestamp
                }));
            }
            MerilEvent::AstmMessageReceived { analyzer_id, message_type, raw_data, timestamp } => {
                log::debug!("ASTM message from {}: {} - {}", analyzer_id, message_type, raw_data);
                
                // Emit event to frontend
                let _ = app.emit("meril:astm-message", serde_json::json!({
                    "analyzer_id": analyzer_id,
                    "message_type": message_type,
                    "raw_data": raw_data,
                    "timestamp": timestamp
                }));
            }
            MerilEvent::LabResultProcessed { analyzer_id, patient_id, test_results, timestamp } => {
                log::info!("Lab results processed for analyzer {}: {} tests", analyzer_id, test_results.len());
                
                // Emit event to frontend
                let _ = app.emit("meril:lab-results", serde_json::json!({
                    "analyzer_id": analyzer_id,
                    "patient_id": patient_id,
                    "test_results": test_results,
                    "timestamp": timestamp
                }));
            }
            MerilEvent::Error { analyzer_id, error, timestamp } => {
                log::error!("Error in analyzer {}: {}", analyzer_id, error);
                
                // Emit event to frontend
                let _ = app.emit("meril:error", serde_json::json!({
                    "analyzer_id": analyzer_id,
                    "error": error,
                    "timestamp": timestamp
                }));
            }
        }
    }
}