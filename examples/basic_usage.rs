//! Basic usage example — demonstrates how the provider is configured.
//!
//! This is a documentation-only example. The provider runs inside wasmCloud
//! and is not started standalone. See local.wadm.yaml for deployment config.

fn main() {
    println!("TCP/UDP Stream Listen Provider — Basic Usage Example");
    println!();
    println!("This provider is designed to run inside wasmCloud.");
    println!("Configure it via link configuration:");
    println!();
    println!("  protocol = tcp | udp");
    println!("  host     = <remote server host>");
    println!("  port     = <remote server port>");
    println!();
    println!("The provider connects to the remote TCP/UDP server and");
    println!("forwards received ASCII messages to linked wasmCloud");
    println!("components via the wasmcloud:messaging/handler interface.");
    println!();
    println!("See local.wadm.yaml and QUICKSTART.md for deployment instructions.");
}
