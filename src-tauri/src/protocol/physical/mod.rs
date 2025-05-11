// Protocol physical layer implementation
// Responsible for socket handling and basic I/O operations

pub mod tcp;

// Re-exports
pub use tcp::TcpConnection;
