// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use anyhow::Result;
use std::sync::Arc;
use storage::repository::db;
use tauri::Emitter;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

// Module declarations
pub mod command;
pub mod handler;
pub mod model;
pub mod protocol;
pub mod service;
pub mod state;
pub mod storage;

use handler::meril_handler::MerilHandler;
use state::AppState;
use storage::get_migrations;
use storage::repository::sqlite::SqliteRepository;

/// Initialize the application
async fn initialize_app() -> Result<(MerilHandler, mpsc::Receiver<String>)> {
    // Initialize components
    let repository = Arc::new(SqliteRepository::new().await?);

    // Create channel for UI events
    let (tx, rx) = mpsc::channel(100);

    // Default port for Meril machine communication
    let default_port = 5060;

    // Create handler with MerilMachineService
    let meril_handler = MerilHandler::new(repository, tx, default_port);

    Ok((meril_handler, rx))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    &format!("sqlite:{}", db::get_data_dir().unwrap().display()),
                    get_migrations(),
                )
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize the app in async context
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                match initialize_app().await {
                    Ok((meril_handler, event_rx)) => {
                        // Create and store app state
                        let app_state = AppState::new(meril_handler);
                        app_handle.manage(app_state);

                        // Start event listener
                        listen_for_events(app_handle, event_rx);
                        // let _ = app_handle.emit("lis_event", "test");
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize app: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::meril_commands::start_meril_service,
            command::meril_commands::stop_meril_service,
            command::meril_commands::get_meril_service_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Listen for events and emit them to the UI
fn listen_for_events(app: AppHandle, mut rx: mpsc::Receiver<String>) {
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            // Emit the event to the UI
            let _ = app.emit("lis_event", event);
        }
    });
}
