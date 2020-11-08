use open_pond_protocol::{parse_config, start_peer_pool, start_servicer};
use std::env;
use std::thread;

/// Thread that starts an active Open Pond node
fn main() -> std::io::Result<()> {
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());

    // Build configuration structure
    let config = parse_config(config_file)?;

    // Start servicer
    let servicer_address = config.servicer.address.clone();
    let servicer_handle = thread::spawn(|| start_servicer(servicer_address));
    println!("Open Pond Node started: {}", config.servicer.name);

    // Start peer pool
    start_peer_pool(config.peers[0].address.clone())?;

    match servicer_handle.join() {
        Ok(_) => (),
        Err(_) => println!("Error shutting down servicer!"),
    };

    Ok(())
}
