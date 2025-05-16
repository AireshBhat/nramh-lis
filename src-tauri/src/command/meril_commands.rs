use crate::state::AppState;
use tauri::{AppHandle, Manager, State};

/// Start the Meril machine service
#[tauri::command]
pub async fn start_meril_service(app_handle: AppHandle, port: u16) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    let meril_handler = state.meril_handler.lock().await;

    match meril_handler.start_service().await {
        Ok(_) => Ok(format!("Meril service started on port {}", port)),
        Err(e) => Err(format!("Failed to start Meril service: {}", e)),
    }
}

/// Stop the Meril machine service
#[tauri::command]
pub async fn stop_meril_service(state: State<'_, AppState>) -> Result<String, String> {
    let meril_handler = state.meril_handler.lock().await;

    match meril_handler.stop_service() {
        Ok(_) => Ok("Meril service stopped".to_string()),
        Err(e) => Err(format!("Failed to stop Meril service: {}", e)),
    }
}

/// Get the status of the Meril machine service
#[tauri::command]
pub async fn get_meril_service_status(state: State<'_, AppState>) -> Result<bool, String> {
    let meril_handler = state.meril_handler.lock().await;

    Ok(meril_handler.is_service_running())
}
