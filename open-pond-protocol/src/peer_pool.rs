use crate::config::Address;
use crate::message::Message;
use std::io::Read;
use std::iter;
use std::net::TcpStream;
use std::thread;
use std::time;

/// Spawns request threads for each peer in the active peer pool
pub fn start_peer_pool(peers: Vec<Address>) -> std::io::Result<()> {
    let mut active: Vec<bool> = iter::repeat(false).take(peers.len()).collect();
    loop {
        thread::sleep(time::Duration::new(1, 0));
        for (i, peer) in peers.iter().enumerate() {
            if !active[i] {
                if let Ok(stream) = TcpStream::connect(peer.address.clone()) {
                    println!("{} accepted connection request", peer.name.clone());
                    thread::spawn(|| request(stream));
                    active[i] = true;
                }
            }
        }
    }
}

fn request(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut response = [0; 1024];
        thread::sleep(time::Duration::new(1, 0));
        if stream.read(&mut response).is_ok() {
            let message: Message = Message::from_bytes((&response).to_vec()).unwrap();
            println!(
                "Received: {}",
                std::str::from_utf8(&message.payload).unwrap()
            );
        }
    }
}
