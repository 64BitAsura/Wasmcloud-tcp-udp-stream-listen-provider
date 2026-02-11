use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::nats::NatsPublisher;

/// Handles incoming messages from TCP/UDP connections
pub struct MessageHandler {
    nats_publisher: Arc<Mutex<NatsPublisher>>,
}

impl MessageHandler {
    /// Create a new message handler
    pub fn new(nats_url: String) -> Self {
        Self {
            nats_publisher: Arc::new(Mutex::new(NatsPublisher::new(nats_url))),
        }
    }

    /// Handle an incoming message from a connection
    pub async fn handle_message(&self, source: &str, message: String) -> Result<()> {
        debug!("Handling message from {}: {} bytes", source, message.len());

        // Trim whitespace and check if message is empty
        let message = message.trim();
        if message.is_empty() {
            debug!("Ignoring empty message from {}", source);
            return Ok(());
        }

        // Forward to NATS
        let mut publisher = self.nats_publisher.lock().await;
        publisher.publish(source, message).await?;

        debug!("Message from {} forwarded to NATS", source);
        Ok(())
    }

    /// Shutdown the message handler
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down message handler");

        let mut publisher = self.nats_publisher.lock().await;
        publisher.disconnect().await?;

        info!("Message handler shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_handler_creation() {
        let _handler = MessageHandler::new("nats://127.0.0.1:4222".to_string());
        // Just verify it can be created without panicking
    }

    #[tokio::test]
    async fn test_handle_empty_message() {
        let handler = MessageHandler::new("nats://127.0.0.1:4222".to_string());
        let result = handler.handle_message("test", "   ".to_string()).await;
        // Should succeed but not publish anything
        assert!(result.is_ok());
    }
}
