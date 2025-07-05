use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Test {
    pub universal_id: String, // Test identifier (e.g., ^^^ALB)
    pub name: String,         // Human readable test name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderPriority {
    Routine,       // "R" in protocol
    Stat,          // "S" in protocol
    AsapEmergency, // "A" in protocol
}

impl From<&str> for OrderPriority {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "S" | "A" => OrderPriority::AsapEmergency,
            _ => OrderPriority::Routine,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionCode {
    Add,     // "A" - Add the requested tests to existing sample
    New,     // "N" - New requests with new sample
    Pending, // "P" - Pending sample (Add but don't schedule)
    Cancel,  // "C" - Cancel request for the tests
}

impl From<&str> for ActionCode {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "A" => ActionCode::Add,
            "N" => ActionCode::New,
            "P" => ActionCode::Pending,
            "C" => ActionCode::Cancel,
            _ => ActionCode::New,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingInfo {
    pub collection_date: Option<DateTime<Utc>>,
    pub received_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestOrder {
    pub id: String,                              // Test Order Identifier
    pub sequence_number: u32,                    // Frame Number
    pub specimen_id: String,                     // Links to Sample model
    pub tests: Vec<Test>,                        // Array of ordered tests
    pub priority: OrderPriority,                 // Priority level
    pub action_code: ActionCode,                 // Action code
    pub ordering_provider: Option<String>,       // Reference to physician
    pub scheduling_info: Option<SchedulingInfo>, // Scheduling information
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
