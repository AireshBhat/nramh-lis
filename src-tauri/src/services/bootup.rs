use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::app_state::AppState;

pub async fn setup<R: tauri::Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let meril_store = app
        .store("meril.json")
        .map_err(|e| format!("Error getting Meril store: {}", e))?;

    let bf6500_store = app
        .store("bf6500.json")
        .map_err(|e| format!("Error getting BF-6500 store: {}", e))?;

    // Initialize AppState with both services
    let mut app_state = AppState::<R>::new(app.clone(), meril_store, bf6500_store)?;

    // Initialize the AppState (handles async operations like auto-starting services)
    app_state.initialize().await?;

    // Store AppState in AppData for global access
    app.manage(app_state);

    log::info!("Bootup service initialized with AppState for Meril and BF-6500 services");
    Ok(())
}
