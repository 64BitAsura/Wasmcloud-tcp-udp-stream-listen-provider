# TCP/UDP Stream Provider Configuration

## Basic TCP Configuration

```json
{
  "protocol": "tcp",
  "host": "192.168.1.100",
  "port": "9000"
}
```

## Basic UDP Configuration

```json
{
  "protocol": "udp",
  "host": "192.168.1.100",
  "port": "5555"
}
```

## Default Values

If no configuration is provided, the provider uses:

```json
{
  "protocol": "tcp",
  "host": "127.0.0.1",
  "port": "9000"
}
```

## wasmCloud Link Configuration

Configuration is provided through link definitions in wadm.yaml:

```yaml
# wadm.yaml example
links:
  - name: stream-link
    from: my-component
    to: tcp-udp-stream-provider
    config:
      - name: provider-config
        properties:
          protocol: tcp
          host: "10.0.0.50"
          port: "9000"
```

## Configuration by Use Case

### Local Development (TCP)

```json
{
  "protocol": "tcp",
  "host": "127.0.0.1",
  "port": "9000"
}
```

### IoT Sensor Stream (UDP)

```json
{
  "protocol": "udp",
  "host": "0.0.0.0",
  "port": "5555"
}
```

### Remote Data Feed (TCP)

```json
{
  "protocol": "tcp",
  "host": "data-feed.example.com",
  "port": "4001"
}
```

## Troubleshooting

### Connection Refused

- Verify the remote server is running and accessible
- Check firewall rules
- Confirm host and port are correct

### No Messages Received

- For TCP: ensure the server sends newline-delimited (`\n`) messages
- For UDP: ensure datagrams are valid UTF-8 text
- Enable debug logging: `RUST_LOG=debug`

### Debug Logging

```bash
RUST_LOG=debug cargo run
```

## Next Steps

- See [README.md](README.md) for features overview
- See [TECHNICAL.md](TECHNICAL.md) for architecture details
- See [QUICKSTART.md](QUICKSTART.md) for deployment guide
