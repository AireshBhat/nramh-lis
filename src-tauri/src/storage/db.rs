use anyhow::{anyhow, Result};
use sqlx::SqlitePool;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize database directories
pub fn init() {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        let data_dir = get_data_dir().expect("Failed to get data directory");
        let db_dir = Path::new(&data_dir).parent().unwrap();

        if !db_dir.exists() {
            std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");
        }

        fs::File::create(&data_dir).unwrap();
    });
}

/// Get the data directory for the application
pub fn get_data_dir() -> Result<PathBuf> {
    init();
    let db_slug = std::env::var("DB_SLUG").unwrap_or_else(|_| "database.db".to_string());
    Ok(Path::new(&db_slug).to_path_buf())
}

/// Establish a connection to the SQLite database
pub async fn establish_connection() -> Result<SqlitePool> {
    let data_dir = get_data_dir()?;

    // Convert to string representation for SQLx
    let db_url = format!("sqlite:{}", data_dir.display());

    log::info!("Database URL: {}", db_url);

    // Create the database connection pool
    let pool = SqlitePool::connect(&db_url).await?;

    Ok(pool)
}
