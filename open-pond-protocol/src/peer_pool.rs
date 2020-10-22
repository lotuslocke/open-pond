use std::io::Read;
use std::net::TcpStream;
use std::thread;
use std::time;

/// Spawns request threads for each peer in the active peer pool
pub fn start_peer_pool(address: String) -> std::io::Result<()> {
    loop {
        thread::sleep(time::Duration::new(1, 0));
        if let Ok(stream) = TcpStream::connect(address.clone()) {
            println!("Peer accepted connection request");
            thread::spawn(|| request(stream));
            break;
        }
    }

    Ok(())
}

fn request(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut response = [0; 1024];
        thread::sleep(time::Duration::new(1, 0));
        if stream.read_exact(&mut response).is_ok() {
            println!("Response: {}", std::str::from_utf8(&response).unwrap());
        } else {
            break;
        }
    }
    Ok(())
}
