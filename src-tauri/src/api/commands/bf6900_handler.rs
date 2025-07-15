use crate::models::{Analyzer, AnalyzerStatus, ConnectionType, Protocol};
use crate::models::hematology::HL7Settings;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tauri::{Emitter, Manager};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct BF6900ConfigResponse {
    pub success: bool,
    pub analyzer: Option<Analyzer>,
    pub hl7_settings: Option<HL7Settings>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BF6900StoreData {
    pub analyzer: Option<Analyzer>,
    pub hl7_settings: Option<HL7Settings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BF6900ServiceStatus {
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

/// Validates BF-6900 analyzer configuration
fn validate_bf6900_config(analyzer: &Analyzer) -> Result<(), String> {
    // Ensure it's TCP/IP connection
    if analyzer.connection_type != ConnectionType::TcpIp {
        return Err("BF-6900 only supports TCP/IP connections".to_string());
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

    // Ensure protocol is HL7 v2.4
    if analyzer.protocol != Protocol::Hl7V24 {
        return Err("BF-6900 only supports HL7 v2.4 protocol".to_string());
    }

    Ok(())
}

/// Validates HL7 settings configuration
fn validate_hl7_settings(settings: &HL7Settings) -> Result<(), String> {
    // Validate timeout
    if settings.timeout_ms == 0 || settings.timeout_ms > 300000 {
        return Err("Timeout must be between 1ms and 300000ms (5 minutes)".to_string());
    }

    // Validate retry attempts
    if settings.retry_attempts > 10 {
        return Err("Retry attempts cannot exceed 10".to_string());
    }

    // Validate encoding
    if settings.encoding != "UTF-8" && settings.encoding != "ASCII" {
        return Err("Encoding must be UTF-8 or ASCII".to_string());
    }

    // Validate supported message types
    if settings.supported_message_types.is_empty() {
        return Err("At least one supported message type is required".to_string());
    }

    for msg_type in &settings.supported_message_types {
        if !msg_type.contains('^') {
            return Err(format!("Invalid HL7 message type format: {}", msg_type));
        }
    }

    Ok(())
}

/// Fetches BF-6900 configuration from the service
/// Returns the current analyzer configuration managed by the BF6900 service
#[tauri::command]
pub async fn fetch_bf6900_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> BF6900ConfigResponse {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Get analyzer config from service
    let analyzer = app_state
        .get_bf6900_service()
        .get_analyzer_config()
        .await;

    log::info!(
        "Successfully fetched BF-6900 configuration from service for analyzer: {}",
        analyzer.id
    );

    // For now, return default HL7 settings since they're not stored in the analyzer model
    let default_hl7_settings = HL7Settings::default();

    BF6900ConfigResponse {
        success: true,
        analyzer: Some(analyzer),
        hl7_settings: Some(default_hl7_settings),
        error_message: None,
    }
}

/// Saves BF-6900 configuration to store
async fn save_bf6900_config_to_store<R: tauri::Runtime>(
    store: &tauri_plugin_store::Store<R>,
    analyzer: &Analyzer,
    hl7_settings: &HL7Settings,
) -> Result<(), String> {
    let store_data = BF6900StoreData {
        analyzer: Some(analyzer.clone()),
        hl7_settings: Some(hl7_settings.clone()),
    };

    let json_value = serde_json::to_value(store_data)
        .map_err(|e| format!("Failed to serialize configuration: {}", e))?;

    store.set("config".to_string(), json_value);

    log::info!(
        "BF-6900 configuration saved successfully for analyzer: {}",
        analyzer.id
    );
    Ok(())
}

/// Updates BF-6900 configuration
#[tauri::command]
pub async fn update_bf6900_config<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    analyzer: Analyzer,
    hl7_settings: HL7Settings,
) -> BF6900ConfigResponse {
    // Validate the analyzer configuration first
    if let Err(validation_error) = validate_bf6900_config(&analyzer) {
        return BF6900ConfigResponse {
            success: false,
            analyzer: None,
            hl7_settings: None,
            error_message: Some(validation_error),
        };
    }

    // Validate HL7 settings
    if let Err(validation_error) = validate_hl7_settings(&hl7_settings) {
        return BF6900ConfigResponse {
            success: false,
            analyzer: None,
            hl7_settings: None,
            error_message: Some(validation_error),
        };
    }

    // Update the timestamp
    let mut updated_analyzer = analyzer;
    updated_analyzer.updated_at = Utc::now();

    // TODO: Add update_analyzer_config method to BF6900 service
    // For now, we'll save to store and log that service update is not yet implemented
    log::warn!("update_bf6900_config: Service update not yet implemented, saving to store directly");

    // Save to store
    let store = match app.store("bf6900.json") {
        Ok(store) => store,
        Err(e) => {
            log::error!("Failed to get bf6900 store: {}", e);
            return BF6900ConfigResponse {
                success: false,
                analyzer: None,
                hl7_settings: None,
                error_message: Some(format!("Failed to access configuration store: {}", e)),
            };
        }
    };

    match save_bf6900_config_to_store(&store, &updated_analyzer, &hl7_settings).await {
        Ok(_) => {
            log::info!(
                "BF-6900 configuration updated successfully for analyzer: {}",
                updated_analyzer.id
            );
            BF6900ConfigResponse {
                success: true,
                analyzer: Some(updated_analyzer),
                hl7_settings: Some(hl7_settings),
                error_message: Some(
                    "Configuration saved to store. Service update not yet implemented.".to_string(),
                ),
            }
        }
        Err(save_error) => BF6900ConfigResponse {
            success: false,
            analyzer: None,
            hl7_settings: None,
            error_message: Some(save_error),
        },
    }
}

/// Gets the status of the BF6900 service
#[tauri::command]
pub async fn get_bf6900_service_status<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<BF6900ServiceStatus, String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();
    let service = app_state.get_bf6900_service();
    let status = service.get_status().await;
    let connections_count = service.get_connections_count().await;
    let is_running = status == AnalyzerStatus::Active;
    
    Ok(BF6900ServiceStatus {
        is_running,
        connections_count,
        analyzer_status: status,
    })
}

/// Starts the BF6900 service
#[tauri::command]
pub async fn start_bf6900_service<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Note: We need mutable access to start the service
    // For now, we'll use a workaround by cloning the service and starting it
    let service = app_state.get_bf6900_service().clone();

    log::info!("Starting BF-6900 service...");

    // Start the service
    match service.start().await {
        Ok(()) => {
            log::info!("BF-6900 service started successfully");

            // Emit event to frontend
            let _ = app.emit(
                "bf6900:service-started",
                serde_json::json!({
                    "timestamp": chrono::Utc::now()
                }),
            );

            Ok(())
        }
        Err(e) => {
            log::error!("Failed to start BF-6900 service: {}", e);

            // Emit error event to frontend
            let _ = app.emit(
                "bf6900:service-error",
                serde_json::json!({
                    "error": e.clone(),
                    "timestamp": chrono::Utc::now()
                }),
            );

            Err(e)
        }
    }
}

/// Stops the BF6900 service
#[tauri::command]
pub async fn stop_bf6900_service<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Get the AppState from AppData
    let app_state = app.state::<crate::app_state::AppState<R>>();

    // Note: We need mutable access to stop the service
    // For now, we'll use a workaround by cloning the service and stopping it
    let service = app_state.get_bf6900_service().clone();

    log::info!("Stopping BF-6900 service...");

    // Stop the service
    match service.stop().await {
        Ok(()) => {
            log::info!("BF-6900 service stopped successfully");

            // Emit event to frontend
            let _ = app.emit(
                "bf6900:service-stopped",
                serde_json::json!({
                    "timestamp": chrono::Utc::now()
                }),
            );

            Ok(())
        }
        Err(e) => {
            log::error!("Failed to stop BF-6900 service: {}", e);

            // Emit error event to frontend
            let _ = app.emit(
                "bf6900:service-error",
                serde_json::json!({
                    "error": e.clone(),
                    "timestamp": chrono::Utc::now()
                }),
            );

            Err(e)
        }
    }
}

/// Creates a default BF-6900 analyzer configuration
fn create_default_bf6900_analyzer() -> Analyzer {
    use uuid::Uuid;

    Analyzer {
        id: Uuid::new_v4().to_string(),
        name: "BF-6900 Hematology Analyzer".to_string(),
        model: "BF-6900".to_string(),
        serial_number: None,
        manufacturer: Some("Mindray".to_string()),
        connection_type: ConnectionType::TcpIp,
        ip_address: Some("192.168.1.100".to_string()),
        port: Some(9100), // Standard HL7 port
        com_port: None,
        baud_rate: None,
        protocol: Protocol::Hl7V24,
        status: AnalyzerStatus::Inactive,
        activate_on_start: false, // Don't auto-start by default
        created_at: Utc::now(),
        updated_at: Utc::now(),
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
        assert!(validate_port(9100));
        assert!(validate_port(65535));
        assert!(!validate_port(0));
    }

    #[test]
    fn test_validate_bf6900_config() {
        let valid_analyzer = create_default_bf6900_analyzer();
        assert!(validate_bf6900_config(&valid_analyzer).is_ok());

        let invalid_analyzer = Analyzer {
            connection_type: ConnectionType::Serial,
            ..valid_analyzer.clone()
        };
        assert!(validate_bf6900_config(&invalid_analyzer).is_err());

        let invalid_protocol = Analyzer {
            protocol: Protocol::Astm,
            ..valid_analyzer.clone()
        };
        assert!(validate_bf6900_config(&invalid_protocol).is_err());
    }

    #[test]
    fn test_validate_hl7_settings() {
        let valid_settings = HL7Settings::default();
        assert!(validate_hl7_settings(&valid_settings).is_ok());

        let invalid_timeout = HL7Settings {
            timeout_ms: 0,
            ..valid_settings.clone()
        };
        assert!(validate_hl7_settings(&invalid_timeout).is_err());

        let invalid_retry = HL7Settings {
            retry_attempts: 15,
            ..valid_settings.clone()
        };
        assert!(validate_hl7_settings(&invalid_retry).is_err());

        let invalid_encoding = HL7Settings {
            encoding: "INVALID".to_string(),
            ..valid_settings.clone()
        };
        assert!(validate_hl7_settings(&invalid_encoding).is_err());

        let invalid_message_type = HL7Settings {
            supported_message_types: vec!["INVALID".to_string()],
            ..valid_settings.clone()
        };
        assert!(validate_hl7_settings(&invalid_message_type).is_err());
    }

    #[test]
    fn test_create_default_bf6900_analyzer() {
        let analyzer = create_default_bf6900_analyzer();
        assert_eq!(analyzer.name, "BF-6900 Hematology Analyzer");
        assert_eq!(analyzer.model, "BF-6900");
        assert_eq!(analyzer.manufacturer, Some("Mindray".to_string()));
        assert_eq!(analyzer.connection_type, ConnectionType::TcpIp);
        assert_eq!(analyzer.protocol, Protocol::Hl7V24);
        assert_eq!(analyzer.port, Some(9100));
        assert!(!analyzer.activate_on_start);
    }
}