# Example: TCP Only Configuration

This example shows how to configure the provider to listen only on TCP.

## Configuration

```yaml
apiVersion: core.oam.dev/v1beta1
kind: Application
metadata:
  name: tcp-only-demo
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
              - name: tcp-config
                properties:
                  enable_tcp: 'true'
                  enable_udp: 'false'
                  tcp_port: '8080'

    - name: tcp-udp-provider
      type: capability
      properties:
        image: file://./provider/build/tcp_udp_stream_provider.par.gz
```

## Testing

Send TCP messages:
```bash
echo "Test message 1" | nc 127.0.0.1 8080
echo "Test message 2" | nc 127.0.0.1 8080
```

Or use the test server:
```bash
cargo run --manifest-path test-server/Cargo.toml -- tcp --count 5
```
