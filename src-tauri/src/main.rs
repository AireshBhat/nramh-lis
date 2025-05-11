// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use nramh_lis_lib::storage::repository::db;
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    db::init();

    // Initialize logging for the application
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Ensure we're running in async context
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tracing::info!("Starting LIS application");
        });

    // Run the Tauri application
    nramh_lis_lib::run()
}
