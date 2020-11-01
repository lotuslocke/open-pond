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
