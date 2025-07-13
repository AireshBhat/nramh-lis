use tauri::async_runtime::block_on;

use crate::services::setup;

pub mod api;
pub mod app_state;
pub mod migrations;
pub mod models;
pub mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations("sqlite:nramh-lis.db", migrations::get_migrations())
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
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
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _ = block_on(setup(app.handle().clone())).map_err(|e| {
                log::error!("Error initializing application: {}", e);
                e
            });
            // let app_data_path = app.path().app_config_dir().unwrap();

            log::info!("Application initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            api::commands::ip_handler::get_local_ip,
            api::commands::meril_handler::fetch_meril_config,
            api::commands::meril_handler::update_meril_config,
            api::commands::meril_handler::get_meril_service_status,
            api::commands::meril_handler::start_meril_service,
            api::commands::meril_handler::stop_meril_service,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
