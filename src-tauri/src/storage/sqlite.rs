use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::model::Error as ModelError;
use crate::storage::traits::{
    StorageService, AnalyzerRepository, PatientRepository, TestResultRepository,
    SampleRepository, TestOrderRepository, UploadRepository, SystemSettingsRepository,
    SystemSetting
};
use crate::storage::repositories::{
    analyzer_repository::SqliteAnalyzerRepository,
    // patient_repository::SqlitePatientRepository,
    // test_result_repository::SqliteTestResultRepository,
    // sample_repository::SqliteSampleRepository,
    // test_order_repository::SqliteTestOrderRepository,
    // upload_repository::SqliteUploadRepository,
    // system_settings_repository::SqliteSystemSettingsRepository,
};

pub struct SqliteStorageService {
    pool: SqlitePool,
    analyzer_repo: SqliteAnalyzerRepository,
    // patient_repo: SqlitePatientRepository,
    // test_result_repo: SqliteTestResultRepository,
    // sample_repo: SqliteSampleRepository,
    // test_order_repo: SqliteTestOrderRepository,
    // upload_repo: SqliteUploadRepository,
    // system_settings_repo: SqliteSystemSettingsRepository,
}

impl SqliteStorageService {
    pub async fn new(pool: SqlitePool) -> Result<Self, ModelError> {
        let analyzer_repo = SqliteAnalyzerRepository::new(pool.clone());
        // let patient_repo = SqlitePatientRepository::new(pool.clone());
        // let test_result_repo = SqliteTestResultRepository::new(pool.clone());
        // let sample_repo = SqliteSampleRepository::new(pool.clone());
        // let test_order_repo = SqliteTestOrderRepository::new(pool.clone());
        // let upload_repo = SqliteUploadRepository::new(pool.clone());
        // let system_settings_repo = SqliteSystemSettingsRepository::new(pool.clone());

        Ok(Self {
            pool,
            analyzer_repo,
            // patient_repo,
            // test_result_repo,
            // sample_repo,
            // test_order_repo,
            // upload_repo,
            // system_settings_repo,
        })
    }

    async fn create_tables(&self) -> Result<(), ModelError> {
        // Create analyzers table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS analyzers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                model TEXT NOT NULL,
                serial_number TEXT,
                manufacturer TEXT,
                connection_type TEXT NOT NULL,
                ip_address TEXT,
                port INTEGER,
                com_port TEXT,
                baud_rate INTEGER,
                protocol TEXT NOT NULL,
                status TEXT NOT NULL,
                activate_on_start BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create patients table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS patients (
                id TEXT PRIMARY KEY,
                last_name TEXT,
                first_name TEXT,
                middle_name TEXT,
                title TEXT,
                birth_date DATETIME,
                sex TEXT NOT NULL,
                street TEXT,
                city TEXT,
                state TEXT,
                zip TEXT,
                country_code TEXT,
                telephone TEXT,
                ordering_physician TEXT,
                attending_physician TEXT,
                referring_physician TEXT,
                height_value REAL,
                height_unit TEXT,
                weight_value REAL,
                weight_unit TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create samples table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS samples (
                id TEXT PRIMARY KEY,
                container_number TEXT,
                container_type TEXT,
                collection_date_time DATETIME,
                collector_id TEXT,
                reception_date_time DATETIME,
                sample_type TEXT NOT NULL,
                status TEXT NOT NULL,
                position TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create test_orders table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS test_orders (
                id TEXT PRIMARY KEY,
                sequence_number INTEGER NOT NULL,
                specimen_id TEXT NOT NULL,
                tests TEXT NOT NULL, -- JSON array of tests
                priority TEXT NOT NULL,
                action_code TEXT NOT NULL,
                ordering_provider TEXT,
                collection_date DATETIME,
                received_date DATETIME,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create test_results table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS test_results (
                id TEXT PRIMARY KEY,
                test_id TEXT NOT NULL,
                sample_id TEXT NOT NULL,
                value TEXT NOT NULL,
                units TEXT,
                reference_range_lower REAL,
                reference_range_upper REAL,
                abnormal_flag TEXT,
                nature_of_abnormality TEXT,
                status TEXT NOT NULL,
                sequence_number INTEGER NOT NULL,
                instrument TEXT,
                completed_date_time DATETIME,
                analyzer_id TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create upload_status table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS upload_status (
                id TEXT PRIMARY KEY,
                result_id TEXT NOT NULL,
                external_system_id TEXT NOT NULL,
                status TEXT NOT NULL,
                upload_date DATETIME,
                response_code TEXT,
                response_message TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        // Create system_settings table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS system_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                description TEXT,
                category TEXT NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
        "#).execute(&self.pool).await?;

        Ok(())
    }

    async fn create_indexes(&self) -> Result<(), ModelError> {
        // Create indexes for better query performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyzers_status ON analyzers(status)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_analyzers_serial ON analyzers(serial_number)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_patients_name ON patients(last_name, first_name)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_patients_birth_date ON patients(birth_date)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_samples_status ON samples(status)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_samples_type ON samples(sample_type)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_samples_position ON samples(position)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_orders_specimen ON test_orders(specimen_id)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_orders_priority ON test_orders(priority)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_results_sample ON test_results(sample_id)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_results_test_id ON test_results(test_id)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_results_analyzer ON test_results(analyzer_id)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_test_results_status ON test_results(status)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_upload_status_result ON upload_status(result_id)").execute(&self.pool).await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_upload_status_status ON upload_status(status)").execute(&self.pool).await?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_system_settings_category ON system_settings(category)").execute(&self.pool).await?;

        Ok(())
    }

    // TODO: Implement when system_settings_repo is available
    // async fn insert_default_settings(&self) -> Result<(), ModelError> {
    //     let default_settings = vec![
    //         ("system.name", "NRAMH LIS", "Laboratory Information System name", "system"),
    //         ("system.version", "2.0.0", "System version", "system"),
    //         ("database.backup.enabled", "true", "Enable automatic database backups", "database"),
    //         ("database.backup.interval_hours", "24", "Backup interval in hours", "database"),
    //         ("upload.retry.max_attempts", "3", "Maximum upload retry attempts", "upload"),
    //         ("upload.retry.delay_seconds", "300", "Delay between upload retries in seconds", "upload"),
    //         ("analyzers.auto_connect", "false", "Automatically connect to analyzers on startup", "analyzers"),
    //         ("results.auto_validate", "true", "Automatically validate test results", "results"),
    //     ];

    //     for (key, value, description, category) in default_settings {
    //         self.system_settings_repo.set_setting(key, value, Some(description), category).await?;
    //     }

    //     Ok(())
    // }
}

#[async_trait::async_trait]
impl StorageService for SqliteStorageService {
    type Error = ModelError;

    // TODO: Implement these repository accessors when repositories are implemented
    // fn analyzers(&self) -> &dyn AnalyzerRepository<Error = Self::Error> {
    //     &self.analyzer_repo
    // }

    // TODO: Implement these repository accessors when repositories are implemented
    // fn patients(&self) -> &dyn PatientRepository<Error = Self::Error> {
    //     &self.patient_repo
    // }

    // fn test_results(&self) -> &dyn TestResultRepository<Error = Self::Error> {
    //     &self.test_result_repo
    // }

    // fn samples(&self) -> &dyn SampleRepository<Error = Self::Error> {
    //     &self.sample_repo
    // }

    // fn test_orders(&self) -> &dyn TestOrderRepository<Error = Self::Error> {
    //     &self.test_order_repo
    // }

    // fn uploads(&self) -> &dyn UploadRepository<Error = Self::Error> {
    //     &self.upload_repo
    // }

    // fn system_settings(&self) -> &dyn SystemSettingsRepository<Error = Self::Error> {
    //     &self.system_settings_repo
    // }

    async fn initialize(&self) -> Result<(), Self::Error> {
        self.create_tables().await?;
        self.create_indexes().await?;
        // TODO: Uncomment when system_settings_repo is implemented
        // self.insert_default_settings().await?;
        Ok(())
    }

    // TODO: Implement these methods when needed
    // async fn migrate(&self) -> Result<(), Self::Error> {
    //     // For now, just reinitialize. In a real system, you'd have proper migration scripts
    //     self.initialize().await
    // }

    // async fn backup(&self) -> Result<(), Self::Error> {
    //     // Simple backup by copying the database file
    //     // In production, you'd want to use SQLite's backup API or external tools
    //     log::info!("Database backup completed");
    //     Ok(())
    // }

    // async fn restore(&self, _backup_path: &str) -> Result<(), Self::Error> {
    //     // Restore from backup
    //     // In production, you'd implement proper restore logic
    //     log::info!("Database restore completed");
    //     Ok(())
    // }
} 