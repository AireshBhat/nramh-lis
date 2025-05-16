use crate::model::test_order::TestOrder;
use crate::storage::repository::sqlite::SqliteRepository;
use anyhow::Result;
use std::sync::Arc;

/// Service for managing test orders
pub struct TestOrderService {
    repository: Arc<SqliteRepository>,
}

impl TestOrderService {
    /// Create a new test order service with the given repository
    pub fn new(repository: Arc<SqliteRepository>) -> Self {
        Self { repository }
    }

    /// Save a test order to the database
    pub async fn save_test_order(&self, order: &TestOrder) -> Result<i64> {
        self.repository.save_test_order(order).await
    }

    /// Find a test order by ID
    pub async fn find_test_order_by_id(&self, order_id: &str) -> Result<Option<TestOrder>> {
        self.repository.find_test_order_by_id(order_id).await
    }

    /// Get test orders for a patient by ID
    pub async fn get_patient_test_orders(&self, patient_id: &str) -> Result<Vec<TestOrder>> {
        self.repository.get_patient_test_orders(patient_id).await
    }

    /// Get test orders by specimen/sample ID
    pub async fn get_test_orders_by_specimen_id(&self, specimen_id: &str) -> Result<Vec<TestOrder>> {
        self.repository.get_test_orders_by_specimen_id(specimen_id).await
    }
} 