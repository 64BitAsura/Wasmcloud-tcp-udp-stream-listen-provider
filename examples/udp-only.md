# Example: UDP Only Configuration

This example shows how to configure the provider to listen only on UDP.

## Configuration

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: udp-only-demo
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
              - name: udp-config
                properties:
                  enable_tcp: 'false'
                  enable_udp: 'true'
                  udp_port: '8081'

    - name: tcp-udp-provider
      type: capability
      properties:
        image: file://./provider/build/tcp_udp_stream_provider.par.gz
```

## Testing

Send UDP messages:
```bash
echo "Test message 1" | nc -u 127.0.0.1 8081
echo "Test message 2" | nc -u 127.0.0.1 8081
```

Or use the test server:
```bash
cargo run --manifest-path test-server/Cargo.toml -- udp --count 5
```
