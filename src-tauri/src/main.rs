// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // // Initialize logging for the application
    // tracing_subscriber::fmt()
    //     .with_span_events(FmtSpan::CLOSE)
    //     .init();

    // Ensure we're running in async context
    // tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async {
    //         log::info!("Starting LIS application");
    //     });

    // Run the Tauri application
    nramh_lis_lib::run()
}
