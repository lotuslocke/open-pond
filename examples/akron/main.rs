use open_pond_api::{new_interface, RequesterEndpoint, ResponseEndpoint, ServicerEndpoint};
use open_pond_protocol::parse_config;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, io, thread, time};

fn main() {
    // Attempt parse of Open Pond configuration file
    let config_file = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/example.toml".to_string());
    let config = parse_config(config_file).unwrap();
    let name = config.local.name;

    // Create application interface objects for making requests and servicing requests
    let (request_endpoint, service_endpoint, response_endpoint) =
        new_interface(config.settings, 0).unwrap();

    // Create tuple to track last message from standard input
    let last_message: Arc<Mutex<(u8, String)>> = Arc::new(Mutex::new((0, "".to_string())));
    let last_message_ref = last_message.clone();
    let name_ref = name.clone();

    // Spawn threads to send requests, service requests, and read responses
    thread::spawn(|| request_updates(request_endpoint, name_ref));
    thread::spawn(|| service(service_endpoint, last_message_ref));
    thread::spawn(|| receive_updates(response_endpoint));

    // Store messages from standard input
    loop {
        thread::sleep(time::Duration::new(1, 0));
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let message = format!("{}: {}", name, input);
            println!("{}", message);
            if let Ok(mut lock) = last_message.lock() {
                lock.0 += 1;
                lock.1 = message;
            }
        }
    }
}

fn request_updates(endpoint: RequesterEndpoint, requester_name: String) {
    loop {
        thread::sleep(time::Duration::new(1, 0));
        endpoint
            .write_request(requester_name.as_bytes().to_vec())
            .unwrap();
    }
}

fn service(endpoint: ServicerEndpoint, last_message: Arc<Mutex<(u8, String)>>) {
    let mut peer_requests: HashMap<String, u8> = HashMap::new();
    loop {
        let (request, return_address) = endpoint.read_request().unwrap();
        let peer_name = String::from_utf8(request).unwrap();

        if let Ok(lock) = last_message.lock() {
            let mut response = vec![0; 1];
            if let Some(request_id) = peer_requests.get_mut(&peer_name) {
                if *request_id != lock.0 {
                    response[0] = 1;
                    response.append(&mut lock.1.as_bytes().to_vec());
                    *request_id = lock.0;
                }
            } else {
                response[0] = 1;
                response.append(&mut lock.1.as_bytes().to_vec());
                peer_requests.insert(peer_name, lock.0);
            }
            endpoint.write_response(return_address, response).unwrap();
        }
    }
}

fn receive_updates(endpoint: ResponseEndpoint) {
    loop {
        let response = endpoint.read_response().unwrap();
        if response[0] != 0 {
            println!(
                "{}",
                String::from_utf8(response[1..response.len()].to_vec()).unwrap()
            );
        }
    }
}
