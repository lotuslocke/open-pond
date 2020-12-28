use open_pond_protocol::ProtocolResult;
use open_pond_protocol::{parse_config, start_requester, start_servicer};
use std::env;
use std::thread;

/// Thread that starts an active Open Pond node
fn main() -> ProtocolResult<()> {
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());

    // Build configuration structure
    let config = parse_config(config_file)?;

    // Start servicer
    let servicer_settings = config.settings.clone();
    let servicer_handle = thread::spawn(|| start_servicer(servicer_settings));
    println!("Open Pond Node started: {}", config.local.name);

    // Start requester
    let requester_settings = config.settings.clone();
    thread::spawn(|| start_requester(requester_settings, config.peers));

    match servicer_handle.join() {
        Ok(_) => (),
        Err(_) => println!("Error shutting down servicer!"),
    };

    Ok(())
}
