// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use anyhow::Result;
use storage::repository::db;
use std::sync::Arc;
use tauri::Emitter;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

// Module declarations
pub mod command;
pub mod handler;
pub mod model;
pub mod protocol;
pub mod service;
pub mod storage;

use command::lis_command::{get_lis_server_status, start_lis_server, stop_lis_server};
use handler::lis_handler::LisHandler;
use protocol::physical::tcp::TcpConfig;
use service::astm::AstmService;
use service::result::ResultService;
use storage::get_migrations;
use storage::repository::sqlite::SqliteRepository;

/// Initialize the application
async fn initialize_app() -> Result<(Arc<LisHandler>, mpsc::Receiver<String>)> {
    // Initialize components
    let repository = Arc::new(SqliteRepository::new().await?);

    let tcp_config = TcpConfig::default();

    let astm_service = Arc::new(AstmService::new(tcp_config));
    let result_service = Arc::new(ResultService::new(repository.clone()));

    // Create channel for UI events
    let (tx, rx) = mpsc::channel(100);

    // Create handler
    let lis_handler = Arc::new(LisHandler::new(astm_service, result_service, tx));

    Ok((lis_handler, rx))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    &format!(
                        "sqlite:{}",
                        db::get_data_dir().unwrap().display()
                    ),
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
                    Ok((lis_handler, event_rx)) => {
                        // Store handler in app state
                        app_handle.manage(lis_handler);

                        // Start event listener
                        listen_for_events(app_handle, event_rx);
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize app: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_lis_server,
            stop_lis_server,
            get_lis_server_status,
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
