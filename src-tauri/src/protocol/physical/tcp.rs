use crate::protocol::constants::{CONNECT_TIMEOUT, READ_TIMEOUT, WRITE_TIMEOUT};
use crate::protocol::error::ProtocolError;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

/// Configuration for TCP connection
#[derive(Debug, Clone)]
pub struct TcpConfig {
    pub host: String,
    pub port: u16,
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(), // Default to all interfaces
            port: 6000,                  // Default port for ASTM
            connect_timeout_ms: CONNECT_TIMEOUT,
            read_timeout_ms: READ_TIMEOUT,
            write_timeout_ms: WRITE_TIMEOUT,
        }
    }
}

/// Represents a TCP connection for ASTM communication
pub struct TcpConnection {
    stream: Arc<Mutex<Option<TcpStream>>>,
    config: TcpConfig,
}

impl TcpConnection {
    /// Create a new TCP connection with the given configuration
    pub fn new(config: TcpConfig) -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            config,
        }
    }

    /// Start the TCP server
    pub async fn start_server(&self) -> Result<(), ProtocolError> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        let listener = TcpListener::bind(&addr).await?;

        tracing::info!("TCP server started on {}", addr);

        // This is a placeholder for the full server implementation
        // In a complete implementation, this would accept connections and handle them
        // For now, we'll just accept a single connection

        let (socket, peer_addr) = listener.accept().await?;
        tracing::info!("New connection from: {}", peer_addr);

        let mut stream_lock = self.stream.lock().await;
        *stream_lock = Some(socket);

        Ok(())
    }

    /// Read data from the TCP connection with timeout
    pub async fn read(&self, buf: &mut [u8]) -> Result<usize, ProtocolError> {
        let mut stream_lock = self.stream.lock().await;

        if let Some(stream) = &mut *stream_lock {
            let read_future = stream.read(buf);
            match timeout(
                Duration::from_millis(self.config.read_timeout_ms),
                read_future,
            )
            .await
            {
                Ok(result) => Ok(result?),
                Err(_) => Err(ProtocolError::Timeout),
            }
        } else {
            Err(ProtocolError::ConnectionClosed)
        }
    }

    /// Write data to the TCP connection with timeout
    pub async fn write(&self, buf: &[u8]) -> Result<usize, ProtocolError> {
        let mut stream_lock = self.stream.lock().await;

        if let Some(stream) = &mut *stream_lock {
            let write_future = stream.write(buf);
            match timeout(
                Duration::from_millis(self.config.write_timeout_ms),
                write_future,
            )
            .await
            {
                Ok(result) => Ok(result?),
                Err(_) => Err(ProtocolError::Timeout),
            }
        } else {
            Err(ProtocolError::ConnectionClosed)
        }
    }

    /// Close the TCP connection
    pub async fn close(&self) -> Result<(), ProtocolError> {
        let mut stream_lock = self.stream.lock().await;

        if let Some(stream) = &mut *stream_lock {
            stream.shutdown().await?;
            *stream_lock = None;
        }

        Ok(())
    }
}
