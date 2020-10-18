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

        loop {
            let mut mail = String::new();
            io::stdin().read_line(&mut mail)?;
            println!("Sending out: {}", mail);
            stream.write(mail.as_bytes())?;
        }
    }
    Ok(())
}

fn request(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut response = [0; 1024];
        thread::sleep(time::Duration::new(1, 0));
        stream.read(&mut response)?;
        println!("Response: {}", std::str::from_utf8(&response).unwrap());
    }
    Ok(())
}
