use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::app_state::AppState;

pub async fn setup<R: tauri::Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let meril_store = app
        .store("meril.json")
        .map_err(|e| format!("Error getting store: {}", e))?;

    // Initialize AppState with AutoQuantMeril service
    let app_state = AppState::<R>::new(app.clone(), meril_store)?;
    
    // Store AppState in AppData for global access
    app.manage(app_state);

    // let _afinion_store = app
    //     .store("afinion.json")
    //     .map_err(|e| format!("Error getting store: {}", e))?;

    // let _bf6500_store = app
    //     .store("bf6500.json")
    //     .map_err(|e| format!("Error getting store: {}", e))?;

    log::info!("Bootup service initialized with AppState");
    Ok(())
}