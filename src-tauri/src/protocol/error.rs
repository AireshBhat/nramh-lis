use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Connection timeout")]
    Timeout,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Invalid checksum: expected {expected}, got {actual}")]
    InvalidChecksum { expected: String, actual: String },

    #[error("Invalid frame format: {0}")]
    InvalidFrameFormat(String),

    #[error("Invalid record format: {0}")]
    InvalidRecordFormat(String),

    #[error("Negative acknowledgment received")]
    NakReceived,

    #[error("Protocol error: {0}")]
    ProtocolError(String),
}
