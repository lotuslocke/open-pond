use open_pond_api::APISocket;
use open_pond_protocol::parse_config;
use std::env;
use std::io;
use std::thread;
use std::time;

fn main() {
    // Attempt parse of Open Pond configuration file
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());
    let config = parse_config(config_file).unwrap();

    // Interacts with servicer endpoint
    let servicer = APISocket::new(config.apps[0].servicer.clone()).unwrap();

    // Interacts with requester endpoint
    let requester = config.apps[0].requester.clone();
    let requester_socket = APISocket::new(requester).unwrap();

    // Spawn thread to read messages
    thread::spawn(|| reader(requester_socket));

    // Write messages from standard input
    loop {
        thread::sleep(time::Duration::new(1, 0));
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let message = format!("{}: {}", config.servicer.name, input);
            servicer.opp_write(message.as_bytes().to_vec()).unwrap();
        }
    }
}

fn reader(reader: APISocket) {
    loop {
        thread::sleep(time::Duration::new(1, 0));
        if reader.opp_request_length().unwrap() > 0 {
            let message = reader.opp_read().unwrap();
            println!("{}", std::str::from_utf8(&message).unwrap());
        }
    }
}
