// Service layer implementations

pub mod machine;
pub mod patient_service;
pub mod result_service;
// pub mod test_order_service;

// Re-export services for easier imports
pub use patient_service::PatientService;
pub use result_service::ResultService;
// pub use test_order_service::TestOrderService;
pub use machine::MerilMachineService;
