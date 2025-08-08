use crate::models::{Analyzer, AnalyzerStatus, ConnectionType, Protocol};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tauri::{Emitter, Manager};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MerilConfigResponse {
    pub success: bool,
    pub analyzer: Option<Analyzer>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerilStoreData {
    pub analyzer: Option<Analyzer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerilServiceStatus {
    pub is_running: bool,
    pub connections_count: usize,
    pub analyzer_status: AnalyzerStatus,
}

/// Validates IP address format
fn validate_ip_address(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

/// Validates port number (1-65535)
fn validate_port(port: u16) -> bool {
    port > 0
}

// Removed unused function - using AppState::create_default_meril_analyzer instead

/// Validates Meril analyzer configuration
fn validate_meril_config(analyzer: &Analyzer) -> Result<(), String> {
    // Ensure it's TCP/IP connection
    if analyzer.connection_type != ConnectionType::TcpIp {
        return Err("Meril AutoQuant only supports TCP/IP connections".to_string());
    }

    // Validate IP address if provided
    if let Some(ip) = &analyzer.ip_address {
        if !validate_ip_address(ip) {
            return Err(format!("Invalid IP address format: {}", ip));
        }
    }

    // Validate port if provided
    if let Some(port) = analyzer.port {
        if !validate_port(port) {
            return Err(format!("Invalid port number: {}", port));
        }
    }

    // Validate external IP address if provided
    if let Some(external_ip) = &analyzer.external_ip {
        if !validate_ip_address(external_ip) {
            return Err(format!("Invalid external IP address format: {}", external_ip));
        }
    }

    // Validate external port if provided
    if let Some(external_port) = analyzer.external_port {
        if !validate_port(external_port) {
            return Err(format!("Invalid external port number: {}", external_port));
        }
    }

    // Ensure protocol is ASTM
    if analyzer.protocol != Protocol::Astm {
        return Err("Meril AutoQuant only supports ASTM protocol".to_string());
    }

    Ok(())
}

/// Fetches Meril AutoQuant configuration from the service
/// Returns the current analyzer configuration managed by the AutoQuantMeril service
#[tauri::command]
pub async fn fetch_meril_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> MerilConfigResponse {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Get analyzer config from service
    let analyzer = app_state
        .get_autoquant_meril_service()
        .get_analyzer_config()
        .await;

    log::info!(
        "Successfully fetched Meril configuration from service for analyzer: {}",
        analyzer.id
    );

    MerilConfigResponse {
        success: true,
        analyzer: Some(analyzer),
        error_message: None,
    }
}

/// Saves Meril configuration to store
async fn save_meril_config_to_store<R: tauri::Runtime>(
    store: &tauri_plugin_store::Store<R>,
    analyzer: &Analyzer,
) -> Result<(), String> {
    let store_data = MerilStoreData {
        analyzer: Some(analyzer.clone()),
    };

    let json_value = serde_json::to_value(store_data)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;

    store.set("config".to_string(), json_value);

    log::info!(
        "Meril configuration saved successfully for analyzer: {}",
        analyzer.id
    );
    Ok(())
}

/// Updates Meril configuration via the service
/// Note: This is a placeholder implementation. In a full implementation,
/// the service would need to be updated to handle configuration changes.
#[tauri::command]
pub async fn update_meril_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    analyzer: Analyzer,
) -> MerilConfigResponse {
    // Validate the configuration first
    if let Err(validation_error) = validate_meril_config(&analyzer) {
        return MerilConfigResponse {
            success: false,
            analyzer: None,
            error_message: Some(validation_error),
        };
    }

    // Update the timestamp
    let mut updated_analyzer = analyzer;
    updated_analyzer.updated_at = Utc::now();

    // TODO: Add update_analyzer_config method to service
    // For now, we'll save to store and log that service update is not yet implemented
    log::warn!("update_meril_config: Service update not yet implemented, saving to store directly");

    // Save to store as fallback (temporary until service update is implemented)
    let store = match app.store("meril.json") {
        Ok(store) => store,
        Err(e) => {
            log::error!("Failed to get meril store: {}", e);
            return MerilConfigResponse {
                success: false,
                analyzer: None,
                error_message: Some(format!("Failed to access configuration store: {}", e)),
            };
        }
    };

    match save_meril_config_to_store(&store, &updated_analyzer).await {
        Ok(_) => {
            log::info!(
                "Meril configuration updated successfully for analyzer: {}",
                updated_analyzer.id
            );
            MerilConfigResponse {
                success: true,
                analyzer: Some(updated_analyzer),
                error_message: Some(
                    "Configuration saved to store. Service update not yet implemented.".to_string(),
                ),
            }
        }
        Err(save_error) => MerilConfigResponse {
            success: false,
            analyzer: None,
            error_message: Some(save_error),
        },
    }
}

/// Gets the status of the AutoQuantMeril service
#[tauri::command]
pub async fn get_meril_service_status<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<MerilServiceStatus, String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    let service = app_state.get_autoquant_meril_service();
    let status = service.get_status().await;
    let connections_count = service.get_connections_count().await;
    let is_running = status == AnalyzerStatus::Active;

    Ok(MerilServiceStatus {
        is_running,
        connections_count,
        analyzer_status: status,
    })
}

/// Starts the AutoQuantMeril service
#[tauri::command]
pub async fn start_meril_service<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Note: We need mutable access to start the service
    // For now, we'll use a workaround by cloning the service and starting it
    let service = app_state.get_autoquant_meril_service().clone();

    log::info!("Starting Meril service...");

    // Start the service
    match service.start().await {
        Ok(()) => {
            log::info!("Meril service started successfully");

            // Emit event to frontend
            let _ = app.emit(
                "meril:service-started",
                serde_json::json!({
                    "timestamp": chrono::Utc::now()
                }),
            );

            Ok(())
        }
        Err(e) => {
            log::error!("Failed to start Meril service: {}", e);

            // Emit error event to frontend
            let _ = app.emit(
                "meril:service-error",
                serde_json::json!({
                    "error": e.clone(),
                    "timestamp": chrono::Utc::now()
                }),
            );

            Err(e)
        }
    }
}

/// Stops the AutoQuantMeril service
#[tauri::command]
pub async fn stop_meril_service<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Note: We need mutable access to stop the service
    // For now, we'll use a workaround by cloning the service and stopping it
    let service = app_state.get_autoquant_meril_service().clone();

    log::info!("Stopping Meril service...");

    // Stop the service
    match service.stop().await {
        Ok(()) => {
            log::info!("Meril service stopped successfully");

            // Emit event to frontend
            let _ = app.emit(
                "meril:service-stopped",
                serde_json::json!({
                    "timestamp": chrono::Utc::now()
                }),
            );

            Ok(())
        }
        Err(e) => {
            log::error!("Failed to stop Meril service: {}", e);

            // Emit error event to frontend
            let _ = app.emit(
                "meril:service-error",
                serde_json::json!({
                    "error": e.clone(),
                    "timestamp": chrono::Utc::now()
                }),
            );

            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ip_address() {
        assert!(validate_ip_address("192.168.1.1"));
        assert!(validate_ip_address("127.0.0.1"));
        assert!(!validate_ip_address("invalid"));
        assert!(!validate_ip_address("256.256.256.256"));
    }

    #[test]
    fn test_validate_port() {
        assert!(validate_port(1));
        assert!(validate_port(5600));
        assert!(validate_port(65535));
        assert!(validate_port(65533));
        assert!(!validate_port(0));
    }

    #[test]
    fn test_validate_meril_config() {
        let valid_analyzer = Analyzer {
            id: "test".to_string(),
            name: "Test".to_string(),
            model: "200i".to_string(),
            serial_number: None,
            manufacturer: Some("Meril".to_string()),
            connection_type: ConnectionType::TcpIp,
            ip_address: Some("192.168.1.1".to_string()),
            port: Some(5600),
            com_port: None,
            baud_rate: None,
            external_ip: None,
            external_port: None,
            protocol: Protocol::Astm,
            status: AnalyzerStatus::Inactive,
            activate_on_start: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(validate_meril_config(&valid_analyzer).is_ok());

        let invalid_analyzer = Analyzer {
            connection_type: ConnectionType::Serial,
            ..valid_analyzer.clone()
        };

        assert!(validate_meril_config(&invalid_analyzer).is_err());

        // Test external IP validation
        let invalid_external_ip = Analyzer {
            external_ip: Some("invalid_ip".to_string()),
            ..valid_analyzer.clone()
        };
        assert!(validate_meril_config(&invalid_external_ip).is_err());

        // Test external port validation
        let invalid_external_port = Analyzer {
            external_port: Some(0),
            ..valid_analyzer.clone()
        };
        assert!(validate_meril_config(&invalid_external_port).is_err());

        // Test valid external fields
        let valid_external = Analyzer {
            external_ip: Some("10.0.0.1".to_string()),
            external_port: Some(8080),
            ..valid_analyzer.clone()
        };
        assert!(validate_meril_config(&valid_external).is_ok());
    }
}
