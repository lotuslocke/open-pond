use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

pub fn start_servicer(address: String) -> std::io::Result<()> {
    let server = TcpListener::bind(address)?;

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepting peer's connection");
                thread::spawn(|| serve(stream));
            }
            Err(_) => println!("An error occured trying to make connection!"),
        };
    }

    Ok(())
}

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

fn serve(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        thread::sleep(time::Duration::new(1, 0));

        let mut mail = String::new();
        if io::stdin().read_line(&mut mail).is_ok() {
            println!("Sending out: {}", mail);
            stream.write_all(mail.as_bytes())?;
        } else {
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
