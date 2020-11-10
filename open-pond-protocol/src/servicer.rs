use crate::message::Message;
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

/// Starts the servicer handling threads
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

fn serve(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        thread::sleep(time::Duration::new(1, 0));

        let mut payload = String::new();
        if io::stdin().read_line(&mut payload).is_ok() {
            println!("Sending out: {}", payload);
            let message = Message::new(1, payload.as_bytes().to_vec()).unwrap();
            stream.write_all(&message.as_bytes().unwrap()).unwrap();
        } else {
            break;
        }
    }
    Ok(())
}
