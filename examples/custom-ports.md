# Example: Custom Ports Configuration

This example shows how to configure custom ports for both TCP and UDP.

## Configuration

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: custom-ports-demo
spec:
  components:
    - name: stream-receiver
      type: component
      properties:
        image: file://./component/build/stream_receiver.wasm
      traits:
        - type: link
          properties:
            target: tcp-udp-provider
            namespace: wasmcloud
            package: messaging
            interfaces: [consumer]
            source_config:
              - name: custom-config
                properties:
                  enable_tcp: 'true'
                  enable_udp: 'true'
                  tcp_port: '9000'
                  udp_port: '9001'

    - name: tcp-udp-provider
      type: capability
      properties:
        image: file://./provider/build/tcp_udp_stream_provider.par.gz
```

## Testing

Send messages to custom ports:

**TCP (port 9000):**
```bash
echo "Custom TCP message" | nc 127.0.0.1 9000
```

**UDP (port 9001):**
```bash
echo "Custom UDP message" | nc -u 127.0.0.1 9001
```

Or use the test server:
```bash
# TCP on port 9000
cargo run --manifest-path test-server/Cargo.toml -- tcp --port 9000 --count 5

# UDP on port 9001
cargo run --manifest-path test-server/Cargo.toml -- udp --port 9001 --count 5

# Both
cargo run --manifest-path test-server/Cargo.toml -- both --tcp-port 9000 --udp-port 9001 --count 5
```
