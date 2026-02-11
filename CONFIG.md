# Configuration Guide

This document describes all configuration options for the TCP/UDP Stream Provider.

## Configuration Format

The provider is configured through link configuration values in JSON format:

```json
{
  "tcp": [ /* TCP connection configurations */ ],
  "udp": [ /* UDP connection configurations */ ]
}
```

## TCP Configuration

Each TCP connection can be configured with the following properties:

### Properties

| Property | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `host` | string | Yes | - | Remote TCP server hostname or IP address |
| `port` | number | Yes | - | Remote TCP server port (1-65535) |
| `reconnect_delay_ms` | number | No | 5000 | Delay in milliseconds before attempting reconnection |

### Examples

#### Single TCP Connection

```json
{
  "tcp": [
    {
      "host": "metrics.example.com",
      "port": 8080
    }
  ]
}
```

#### Multiple TCP Connections

```json
{
  "tcp": [
    {
      "host": "server1.example.com",
      "port": 8080,
      "reconnect_delay_ms": 3000
    },
    {
      "host": "server2.example.com",
      "port": 8081,
      "reconnect_delay_ms": 10000
    }
  ]
}
```

#### TCP with Custom Reconnect Delay

```json
{
  "tcp": [
    {
      "host": "unreliable.example.com",
      "port": 8080,
      "reconnect_delay_ms": 30000
    }
  ]
}
```

## UDP Configuration

Each UDP connection can be configured with the following properties:

### Properties

| Property | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `host` | string | Yes | - | Remote UDP server hostname or IP address |
| `port` | number | Yes | - | Remote UDP server port (1-65535) |

### Examples

#### Single UDP Connection

```json
{
  "udp": [
    {
      "host": "logs.example.com",
      "port": 9090
    }
  ]
}
```

#### Multiple UDP Connections

```json
{
  "udp": [
    {
      "host": "syslog1.example.com",
      "port": 514
    },
    {
      "host": "syslog2.example.com",
      "port": 514
    }
  ]
}
```

## Combined Configuration

You can configure both TCP and UDP connections simultaneously:

```json
{
  "tcp": [
    {
      "host": "metrics.example.com",
      "port": 8080,
      "reconnect_delay_ms": 5000
    },
    {
      "host": "events.example.com",
      "port": 8081
    }
  ],
  "udp": [
    {
      "host": "logs.example.com",
      "port": 9090
    },
    {
      "host": "telemetry.example.com",
      "port": 9091
    }
  ]
}
```

## Message Format

Messages received from TCP/UDP streams are forwarded to NATS with the following JSON structure:

```json
{
  "source": "tcp://metrics.example.com:8080",
  "content": "Message content as ASCII string",
  "timestamp": 1234567890
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `source` | string | Connection source in format `tcp://host:port` or `udp://host:port` |
| `content` | string | The message content received from the stream (ASCII) |
| `timestamp` | integer | Unix timestamp when the message was received |

## NATS Topic Routing

Messages are published to NATS topics based on the source address:

### Topic Format

```
wasmcloud.stream.messages.<sanitized_source>
```

### Sanitization Rules

The source address is sanitized for NATS topic compatibility:
- `://` becomes `_`
- `:` becomes `_`
- `/` becomes `_`
- `.` becomes `_`

### Examples

| Source | NATS Topic |
|--------|------------|
| `tcp://localhost:8080` | `wasmcloud.stream.messages.tcp_localhost_8080` |
| `udp://192.168.1.1:9090` | `wasmcloud.stream.messages.udp_192_168_1_1_9090` |
| `tcp://example.com:8080` | `wasmcloud.stream.messages.tcp_example_com_8080` |

## Connection Behavior

### TCP Connections

- **Connection Establishment**: Provider acts as a TCP client and connects to the remote server
- **Message Reading**: Continuously reads from the TCP stream with a 4KB buffer
- **Reconnection**: Automatically attempts to reconnect on connection failure (framework in place)
- **Connection Close**: Gracefully handles when the remote server closes the connection

### UDP Connections

- **Socket Binding**: Provider binds a local UDP socket
- **Connection**: Connects to the remote UDP server endpoint
- **Message Reading**: Continuously receives datagrams with a 4KB buffer
- **No Reconnection**: UDP is connectionless; socket remains open

## Link Configuration in WasmCloud

When creating a link in WasmCloud, provide the configuration as values:

### Using wash CLI

```bash
wash link put <component-id> <provider-id> \
  --link-name default \
  --values '{
    "tcp": [{"host": "localhost", "port": 8080}],
    "udp": [{"host": "localhost", "port": 9090}]
  }'
```

### Using WADM Manifest

```yaml
- type: link
  properties:
    target: tcp-udp-provider
    namespace: wasmcloud
    package: stream
    interfaces: [messages]
    target_config:
      - name: stream-config
        properties:
          tcp:
            - host: localhost
              port: 8080
              reconnect_delay_ms: 5000
          udp:
            - host: localhost
              port: 9090
```

## Environment Variables

The provider uses these environment variables (set by WasmCloud):

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Logging level (e.g., `info`, `debug`, `trace`) |
| `WASMCLOUD_RPC_HOST` | NATS server host (default: `127.0.0.1`) |
| `WASMCLOUD_RPC_PORT` | NATS server port (default: `4222`) |

## Best Practices

### Connection Limits

- **Keep connections reasonable**: Each connection spawns an async task
- **Monitor resource usage**: Multiple connections consume memory and file descriptors
- **Use appropriate buffer sizes**: Default 4KB is suitable for most use cases

### Reconnection Strategy

- **Short delays for reliable networks**: Use 1000-5000ms for stable connections
- **Longer delays for unstable networks**: Use 10000-30000ms to avoid overwhelming servers
- **Consider exponential backoff**: Future enhancement planned

### Network Considerations

- **Firewall rules**: Ensure provider can reach remote servers
- **DNS resolution**: Use IP addresses if DNS is unreliable
- **Connection timeouts**: TCP connections may hang; consider timeout configuration (future enhancement)

### Message Handling

- **ASCII content**: Provider expects ASCII text messages
- **Large messages**: Messages are buffered; very large messages may be truncated
- **Empty messages**: Whitespace-only messages are filtered out

## Limitations

- **Unidirectional only**: Provider receives messages but cannot send replies (deferred feature)
- **No TLS/SSL**: Plain TCP/UDP only (future enhancement)
- **No authentication**: No built-in authentication mechanism (future enhancement)
- **ASCII only**: Binary message support limited (JSON serialization)
- **Fixed buffer size**: 4KB buffer for message reading

## Troubleshooting

### Configuration Not Applied

- Verify JSON syntax is valid
- Check provider logs for parsing errors
- Ensure all required fields are present

### Connection Failures

- Verify host and port are correct
- Check network connectivity: `nc -zv <host> <port>`
- Review firewall rules
- Check provider logs for error messages

### Messages Not Received

- Verify remote server is sending data
- Check NATS subscription is active
- Enable debug logging: `RUST_LOG=debug`
- Monitor NATS topics: `nats sub "wasmcloud.stream.messages.>"`

## See Also

- [README.md](README.md) - Main documentation
- [QUICKSTART.md](QUICKSTART.md) - Quick start guide
- [Agents.md](Agents.md) - Architecture and implementation details
- [examples/](examples/) - Usage examples
