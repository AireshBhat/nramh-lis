use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::infrastructure::error::Error as ModelError;
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

/// Repository Manager coordinates access to all repositories
/// This is the main entry point for data access operations
pub struct RepositoryManager {
    analyzer_repo: Arc<SqliteAnalyzerRepository>,
    // patient_repo: Arc<SqlitePatientRepository>,
    // test_result_repo: Arc<SqliteTestResultRepository>,
    // sample_repo: Arc<SqliteSampleRepository>,
    // test_order_repo: Arc<SqliteTestOrderRepository>,
    // upload_repo: Arc<SqliteUploadRepository>,
    // system_settings_repo: Arc<SqliteSystemSettingsRepository>,
}

impl RepositoryManager {
    /// Create a new repository manager with all repositories
    pub async fn new(pool: SqlitePool) -> Result<Self, ModelError> {
        let analyzer_repo = Arc::new(SqliteAnalyzerRepository::new(pool.clone()));
        // let patient_repo = Arc::new(SqlitePatientRepository::new(pool.clone()));
        // let test_result_repo = Arc::new(SqliteTestResultRepository::new(pool.clone()));
        // let sample_repo = Arc::new(SqliteSampleRepository::new(pool.clone()));
        // let test_order_repo = Arc::new(SqliteTestOrderRepository::new(pool.clone()));
        // let upload_repo = Arc::new(SqliteUploadRepository::new(pool.clone()));
        // let system_settings_repo = Arc::new(SqliteSystemSettingsRepository::new(pool.clone()));

        Ok(Self {
            analyzer_repo,
            // patient_repo,
            // test_result_repo,
            // sample_repo,
            // test_order_repo,
            // upload_repo,
            // system_settings_repo,
        })
    }

    /// Get analyzer repository
    pub fn analyzers(&self) -> Arc<dyn AnalyzerRepository<Error = ModelError, Id = String> + Send + Sync> {
        Arc::clone(&self.analyzer_repo) as Arc<dyn AnalyzerRepository<Error = ModelError, Id = String> + Send + Sync>
    }

    /// Get patient repository
    // pub fn patients(&self) -> Arc<dyn PatientRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.patient_repo) as Arc<dyn PatientRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Get test result repository
    // pub fn test_results(&self) -> Arc<dyn TestResultRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.test_result_repo) as Arc<dyn TestResultRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Get sample repository
    // pub fn samples(&self) -> Arc<dyn SampleRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.sample_repo) as Arc<dyn SampleRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Get test order repository
    // pub fn test_orders(&self) -> Arc<dyn TestOrderRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.test_order_repo) as Arc<dyn TestOrderRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Get upload repository
    // pub fn uploads(&self) -> Arc<dyn UploadRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.upload_repo) as Arc<dyn UploadRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Get system settings repository
    // pub fn system_settings(&self) -> Arc<dyn SystemSettingsRepository<Error = ModelError, Id = String> + Send + Sync> {
    //     Arc::clone(&self.system_settings_repo) as Arc<dyn SystemSettingsRepository<Error = ModelError, Id = String> + Send + Sync>
    // }

    /// Initialize all repositories
    pub async fn initialize(&self) -> Result<(), ModelError> {
        // Initialize each repository if needed
        // For now, just return Ok since repositories are initialized in their constructors
        Ok(())
    }

    /// Get repository health status
    pub async fn health_check(&self) -> Result<RepositoryHealth, ModelError> {
        let mut health = RepositoryHealth::default();
        
        // Check analyzer repository health
        // match self.analyzer_repo.list(None, None).await {
        //     Ok(_) => health.analyzer_repo = RepositoryStatus::Healthy,
        //     Err(_) => health.analyzer_repo = RepositoryStatus::Unhealthy,
        // }

        // TODO: Add health checks for other repositories when implemented
        
        Ok(health)
    }

    /// Get repository statistics
    pub async fn get_statistics(&self) -> Result<RepositoryStatistics, ModelError> {
        let stats = RepositoryStatistics::default();
        
        // Get analyzer statistics
        // match self.analyzer_repo.list(None, None).await {
        //     Ok(analyzers) => {
        //         stats.total_analyzers = analyzers.len();
        //         stats.active_analyzers = analyzers.iter()
        //             .filter(|a| a.status == crate::domain::entities::analyzer::AnalyzerStatus::Active)
        //             .count();
        //     },
        //     Err(_) => {
        //         stats.total_analyzers = 0;
        //         stats.active_analyzers = 0;
        //     }
        // }

        // TODO: Add statistics for other repositories when implemented
        
        Ok(stats)
    }
}

/// Health status for each repository
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Overall repository health status
#[derive(Debug, Clone)]
pub struct RepositoryHealth {
    pub analyzer_repo: RepositoryStatus,
    pub patient_repo: RepositoryStatus,
    pub test_result_repo: RepositoryStatus,
    pub sample_repo: RepositoryStatus,
    pub test_order_repo: RepositoryStatus,
    pub upload_repo: RepositoryStatus,
    pub system_settings_repo: RepositoryStatus,
}

impl Default for RepositoryHealth {
    fn default() -> Self {
        Self {
            analyzer_repo: RepositoryStatus::Unknown,
            patient_repo: RepositoryStatus::Unknown,
            test_result_repo: RepositoryStatus::Unknown,
            sample_repo: RepositoryStatus::Unknown,
            test_order_repo: RepositoryStatus::Unknown,
            upload_repo: RepositoryStatus::Unknown,
            system_settings_repo: RepositoryStatus::Unknown,
        }
    }
}

/// Repository statistics
#[derive(Debug, Clone)]
pub struct RepositoryStatistics {
    pub total_analyzers: usize,
    pub active_analyzers: usize,
    pub total_patients: usize,
    pub total_samples: usize,
    pub total_test_results: usize,
    pub total_test_orders: usize,
    pub pending_uploads: usize,
}

impl Default for RepositoryStatistics {
    fn default() -> Self {
        Self {
            total_analyzers: 0,
            active_analyzers: 0,
            total_patients: 0,
            total_samples: 0,
            total_test_results: 0,
            total_test_orders: 0,
            pending_uploads: 0,
        }
    }
}

/// Transaction coordinator for cross-repository operations
pub struct TransactionCoordinator {
    pool: SqlitePool,
}

impl TransactionCoordinator {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Execute a transaction across multiple repositories
    pub async fn execute_transaction<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: for<'a> FnOnce(&'a mut sqlx::Transaction<'_, sqlx::Sqlite>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'a>>,
        E: From<sqlx::Error>,
    {
        let mut transaction = self.pool.begin().await?;
        let result = f(&mut transaction).await?;
        transaction.commit().await?;
        Ok(result)
    }
}

// Legacy StorageService implementation for backward compatibility
pub struct SqliteStorageService {
    repository_manager: RepositoryManager,
}

impl SqliteStorageService {
    pub async fn new(pool: SqlitePool) -> Result<Self, ModelError> {
        let repository_manager = RepositoryManager::new(pool).await?;
        Ok(Self { repository_manager })
    }
}

#[async_trait::async_trait]
impl StorageService for SqliteStorageService {
    type Error = ModelError;

    async fn initialize(&self) -> Result<(), Self::Error> {
        self.repository_manager.initialize().await
    }
} 