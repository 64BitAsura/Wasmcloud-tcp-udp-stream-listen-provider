use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 9000;

const CONFIG_PROTOCOL: &str = "protocol";
const CONFIG_HOST: &str = "host";
const CONFIG_PORT: &str = "port";
const CONFIG_SUBSCRIPTIONS: &str = "subscriptions";

/// Supported stream protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum StreamProtocol {
    /// TCP stream client
    #[default]
    Tcp,
    /// UDP datagram client
    Udp,
}

/// Configuration for connecting to a remote TCP/UDP server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionConfig {
    /// Stream protocol (tcp or udp)
    #[serde(default)]
    pub protocol: StreamProtocol,

    /// Remote server host
    #[serde(default = "default_host")]
    pub host: String,

    /// Remote server port
    #[serde(default = "default_port")]
    pub port: u16,

    /// List of topics/subjects to use when forwarding messages to components
    #[serde(default)]
    pub subscriptions: Vec<String>,
}

fn default_host() -> String {
    DEFAULT_HOST.to_string()
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        ConnectionConfig {
            protocol: StreamProtocol::Tcp,
            host: default_host(),
            port: default_port(),
            subscriptions: vec![],
        }
    }
}

impl ConnectionConfig {
    /// Return the remote address as "host:port"
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Merge a given [`ConnectionConfig`] with another, coalescing fields and overriding
    /// where necessary
    pub fn merge(&self, extra: ConnectionConfig) -> ConnectionConfig {
        let mut out = self.clone();
        if extra.protocol != StreamProtocol::default() {
            out.protocol = extra.protocol;
        }
        if extra.host != default_host() {
            out.host = extra.host;
        }
        if extra.port != default_port() {
            out.port = extra.port;
        }
        if !extra.subscriptions.is_empty() {
            out.subscriptions = extra.subscriptions;
        }
        out
    }
}

impl From<&HashMap<String, String>> for ConnectionConfig {
    /// Construct configuration from the passed config values
    fn from(values: &HashMap<String, String>) -> ConnectionConfig {
        let mut config = ConnectionConfig::default();

        if let Some(proto) = values.get(CONFIG_PROTOCOL) {
            config.protocol = match proto.to_lowercase().as_str() {
                "udp" => StreamProtocol::Udp,
                _ => StreamProtocol::Tcp,
            };
        }
        if let Some(host) = values.get(CONFIG_HOST) {
            config.host = host.to_string();
        }
        if let Some(port) = values.get(CONFIG_PORT) {
            if let Ok(p) = port.parse::<u16>() {
                config.port = p;
            }
        }
        if let Some(sub) = values.get(CONFIG_SUBSCRIPTIONS) {
            config
                .subscriptions
                .extend(sub.split(',').map(|s| s.to_string()));
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ConnectionConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(config.protocol, StreamProtocol::Tcp);
        assert!(config.subscriptions.is_empty());
    }

    #[test]
    fn test_from_map_empty() {
        let config = ConnectionConfig::from(&HashMap::new());
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 9000);
        assert_eq!(config.protocol, StreamProtocol::Tcp);
    }

    #[test]
    fn test_from_map_custom() {
        let mut map = HashMap::new();
        map.insert("protocol".to_string(), "udp".to_string());
        map.insert("host".to_string(), "10.0.0.1".to_string());
        map.insert("port".to_string(), "5555".to_string());
        map.insert("subscriptions".to_string(), "topic.a,topic.b".to_string());

        let config = ConnectionConfig::from(&map);
        assert_eq!(config.protocol, StreamProtocol::Udp);
        assert_eq!(config.host, "10.0.0.1");
        assert_eq!(config.port, 5555);
        assert_eq!(config.subscriptions, vec!["topic.a", "topic.b"]);
    }

    #[test]
    fn test_addr() {
        let config = ConnectionConfig {
            host: "192.168.1.10".to_string(),
            port: 8080,
            ..Default::default()
        };
        assert_eq!(config.addr(), "192.168.1.10:8080");
    }

    #[test]
    fn test_merge() {
        let base = ConnectionConfig {
            protocol: StreamProtocol::Tcp,
            host: "localhost".to_string(),
            port: 9000,
            subscriptions: vec!["topic.default".to_string()],
        };

        let extra = ConnectionConfig {
            protocol: StreamProtocol::Udp,
            host: "10.0.0.5".to_string(),
            port: 7777,
            subscriptions: vec!["topic.override".to_string()],
        };

        let merged = base.merge(extra);
        assert_eq!(merged.protocol, StreamProtocol::Udp);
        assert_eq!(merged.host, "10.0.0.5");
        assert_eq!(merged.port, 7777);
        assert_eq!(merged.subscriptions, vec!["topic.override"]);
    }

    #[test]
    fn test_merge_preserves_defaults() {
        let base = ConnectionConfig {
            protocol: StreamProtocol::Udp,
            host: "10.0.0.1".to_string(),
            port: 5555,
            subscriptions: vec!["topic.a".to_string()],
        };

        let extra = ConnectionConfig::default();
        let merged = base.merge(extra);
        // Default values should not override non-default base values
        assert_eq!(merged.protocol, StreamProtocol::Udp);
        assert_eq!(merged.host, "10.0.0.1");
        assert_eq!(merged.port, 5555);
        // Empty subscriptions should not replace existing
        assert_eq!(merged.subscriptions, vec!["topic.a"]);
    }
}
