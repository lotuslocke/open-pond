use std::env;
use std::thread;

use open_pond_protocol::{start_peer_pool, start_servicer};

fn main() -> std::io::Result<()> {
    let server_address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let peer_address = env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:8081".to_string());

    let servicer_handle = thread::spawn(|| start_servicer(server_address));
    println!("Server spawned");

    start_peer_pool(peer_address)?;

    match servicer_handle.join() {
        Ok(_) => (),
        Err(_) => println!("Error shutting down servicer!"),
    };

    Ok(())
}
