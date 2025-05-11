use crate::protocol::physical::tcp::TcpConfig;
use crate::service::astm::AstmService;
use crate::service::result::ResultService;
use anyhow::{anyhow, Result};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

/// Handler for LIS communications
pub struct LisHandler {
    astm_service: Arc<AstmService>,
    result_service: Arc<ResultService>,
    // Channel for sending events to the UI
    event_sender: mpsc::Sender<String>,
}

impl LisHandler {
    /// Create a new LIS handler
    pub fn new(
        astm_service: Arc<AstmService>,
        result_service: Arc<ResultService>,
        event_sender: mpsc::Sender<String>,
    ) -> Self {
        Self {
            astm_service,
            result_service,
            event_sender,
        }
    }

    /// Start the LIS server with the given configuration
    pub async fn start_server(&self, config: TcpConfig) -> Result<()> {
        tracing::info!("Starting LIS server with config: {:?}", config);

        // Send event to UI
        self.send_event(&format!(
            "Starting LIS server on {}:{}",
            config.host, config.port
        ))
        .await?;

        // Start the TCP server
        self.astm_service.start_server().await?;

        // Send event to UI
        self.send_event(&format!(
            "LIS server started on {}:{}",
            config.host, config.port
        ))
        .await?;

        Ok(())
    }

    /// Handle a connection from an analyzer
    pub async fn handle_connection(&self) -> Result<()> {
        tracing::info!("Handling new analyzer connection");

        // Send event to UI
        self.send_event("New analyzer connection received").await?;

        // Handle the connection using the ASTM service
        self.astm_service.handle_connection().await?;

        // Send event to UI
        self.send_event("Analyzer connection processing complete")
            .await?;

        Ok(())
    }

    /// Send an event to the UI
    async fn send_event(&self, message: &str) -> Result<()> {
        if let Err(e) = self.event_sender.send(message.to_string()).await {
            tracing::error!("Failed to send event to UI: {}", e);
            return Err(anyhow!("Failed to send event to UI: {}", e));
        }

        Ok(())
    }
}
