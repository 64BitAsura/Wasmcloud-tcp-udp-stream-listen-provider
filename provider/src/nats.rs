use anyhow::{Context, Result};
use async_nats::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Message forwarded to NATS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessage {
    /// Source address of the message (e.g., "tcp://host:port")
    pub source: String,
    /// The actual message content
    pub content: String,
    /// Timestamp when message was received
    pub timestamp: i64,
}

/// NATS publisher for forwarding messages
pub struct NatsPublisher {
    nats_url: String,
    client: Option<Client>,
    /// Base topic for publishing messages
    base_topic: String,
}

impl NatsPublisher {
    /// Create a new NATS publisher
    pub fn new(nats_url: String) -> Self {
        Self {
            nats_url,
            client: None,
            base_topic: "wasmcloud.stream.messages".to_string(),
        }
    }

    /// Connect to NATS server
    async fn connect(&mut self) -> Result<()> {
        if self.client.is_some() {
            debug!("Already connected to NATS");
            return Ok(());
        }

        info!("Connecting to NATS at {}", self.nats_url);

        let client = async_nats::connect(&self.nats_url)
            .await
            .context("Failed to connect to NATS")?;

        self.client = Some(client);
        info!("Connected to NATS successfully");
        Ok(())
    }

    /// Publish a message to NATS
    pub async fn publish(&mut self, source: &str, content: &str) -> Result<()> {
        // Ensure we're connected
        if self.client.is_none() {
            self.connect().await?;
        }

        let client = self.client.as_ref().unwrap();

        // Create message structure
        let message = StreamMessage {
            source: source.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        // Serialize to JSON
        let payload = serde_json::to_vec(&message).context("Failed to serialize message")?;

        // Publish to NATS
        let topic = format!("{}.{}", self.base_topic, sanitize_topic(source));

        debug!("Publishing to topic: {}", topic);
        client
            .publish(topic, payload.into())
            .await
            .context("Failed to publish to NATS")?;

        Ok(())
    }

    /// Disconnect from NATS
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(client) = self.client.take() {
            info!("Disconnecting from NATS");
            // async-nats client doesn't need explicit disconnect
            drop(client);
        }
        Ok(())
    }

    /// Set custom base topic
    #[allow(dead_code)]
    pub fn set_base_topic(&mut self, topic: String) {
        self.base_topic = topic;
    }
}

/// Sanitize source string to be a valid NATS topic segment
fn sanitize_topic(source: &str) -> String {
    source
        .replace("://", "_")
        .replace([':', '/', '.'], "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_topic() {
        assert_eq!(sanitize_topic("tcp://localhost:8080"), "tcp_localhost_8080");
        assert_eq!(
            sanitize_topic("udp://example.com:9090"),
            "udp_example_com_9090"
        );
        assert_eq!(sanitize_topic("192.168.1.1:8080"), "192_168_1_1_8080");
    }

    #[test]
    fn test_stream_message_serialization() {
        let msg = StreamMessage {
            source: "tcp://localhost:8080".to_string(),
            content: "test message".to_string(),
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("tcp://localhost:8080"));
        assert!(json.contains("test message"));
        assert!(json.contains("1234567890"));
    }

    #[tokio::test]
    async fn test_nats_publisher_creation() {
        let publisher = NatsPublisher::new("nats://127.0.0.1:4222".to_string());
        assert_eq!(publisher.nats_url, "nats://127.0.0.1:4222");
        assert_eq!(publisher.base_topic, "wasmcloud.stream.messages");
    }
}
