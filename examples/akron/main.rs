use open_pond_api::{new_interface, RequesterEndpoint, ServicerEndpoint};
use open_pond_protocol::parse_config;

use std::sync::{Arc, Mutex};
use std::{env, io, thread, time};

fn main() {
    // Attempt parse of Open Pond configuration file
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());
    let config = parse_config(config_file).unwrap();

    // Create application interface objects for making requests and servicing requests
    let (request_endpoint, service_endpoint) = new_interface(config.settings, 0).unwrap();

    // Create tuple to track last message from standard input
    let last_message: Arc<Mutex<(u8, String)>> = Arc::new(Mutex::new((0, "".to_string())));
    let last_message_ref = last_message.clone();

    // Spawn threads to send and respond to requests
    thread::spawn(|| request(request_endpoint));
    thread::spawn(|| service(service_endpoint, last_message_ref));

    // Store messages from standard input
    loop {
        thread::sleep(time::Duration::new(1, 0));
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let message = format!("{}: {}", config.local.name, input);
            println!("{}", message);
            if let Ok(mut lock) = last_message.lock() {
                lock.0 += 1;
                lock.1 = message;
            }
        }
    }
}

fn request(endpoint: RequesterEndpoint) {
    let mut last_request: u8 = 0;
    loop {
        thread::sleep(time::Duration::new(1, 0));
        endpoint.write_request([last_request].to_vec()).unwrap();
        let response = endpoint.read_response().unwrap();
        if response[0] != last_request {
            last_request = response[0];
            println!(
                "{}",
                String::from_utf8(response[1..response.len()].to_vec()).unwrap()
            );
        }
    }
}

fn service(endpoint: ServicerEndpoint, last_message: Arc<Mutex<(u8, String)>>) {
    loop {
        thread::sleep(time::Duration::new(0, 100));
        let (request, return_address) = endpoint.read_request().unwrap();
        if let Ok(lock) = last_message.lock() {
            let mut response = vec![lock.0; 1];
            if request[0] != lock.0 {
                response.append(&mut lock.1.as_bytes().to_vec());
            }
            endpoint.write_response(return_address, response).unwrap();
        }
    }
}
