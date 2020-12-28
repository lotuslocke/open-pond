use crate::config::Settings;
use crate::message::Message;
use crate::protocol::ProtocolResult;

use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;

/// Starts the servicer handling threads
pub fn start_servicer(settings: Settings) -> ProtocolResult<()> {
    // Generate servicer for this portal
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", settings.servicer))?;
    let manager_port = settings.servicer_manager;
    thread::spawn(move || servicer(socket, manager_port));

    // Generate manager for servicing requests between applications
    let endpoint_socket = UdpSocket::bind(format!("0.0.0.0:{}", settings.servicer_manager))?;
    thread::spawn(|| servicer_manager(endpoint_socket));

    Ok(())
}

/// Thread that handles the servicer
pub fn servicer(socket: UdpSocket, manager_port: u16) -> ProtocolResult<()> {
    loop {
        // Receive message from another peer
        let mut request = [0; 1024];
        let (len, address) = socket.recv_from(&mut request)?;

        // Validate message and spawn response handler
        let message = Message::from_bytes(request[0..len].to_vec())?;
        let return_address = format!("{}:{}", address.ip(), message.port);
        thread::spawn(move || response_handler(message.payload, return_address, manager_port));
    }
}

/// These threads keeps track of responses to requests
pub fn response_handler(
    payload: Vec<u8>,
    return_address: String,
    manager_port: u16,
) -> ProtocolResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Send message to application servicer manager
    let manager_address = format!("0.0.0.0:{}", manager_port);
    socket.send_to(&payload, manager_address)?;

    // Recieve response back from application and send back to peers
    let mut response = [0; 1019];
    let (len, _) = socket.recv_from(&mut response)?;
    let message = Message::new(response[0], 0, response[1..len].to_vec())?;
    socket.send_to(&message.as_bytes()?, return_address)?;
    Ok(())
}

// Process that takes application requests and stores them until the application is ready to service them
fn servicer_manager(socket: UdpSocket) -> ProtocolResult<()> {
    // Avoid stalling waiting for requests that may not come
    socket.set_nonblocking(true)?;

    // Setup mailbox storage for application responses
    let mut mailboxes: HashMap<u8, Vec<Message>> = HashMap::new();

    loop {
        let mut response = [0; 1024];

        // Place incoming responses into mailbox that corresponds with application
        socket.recv_from(&mut response)?;
        let message = Message::from_bytes(response.to_vec())?;
        if let Some(mailbox) = mailboxes.get_mut(&message.id) {
            mailbox.push(message);
        } else {
            let mut mailbox = Vec::new();
            let id = message.id;
            mailbox.push(message);
            mailboxes.insert(id, mailbox);
        }

        // See if there is request for data from applications
        let mut app_id = [0; 1];
        if let Ok((_, address)) = socket.recv_from(&mut app_id) {
            if let Some(mailbox) = mailboxes.get_mut(&app_id[0]) {
                let request = mailbox.remove(0);
                socket.send_to(&request.as_bytes()?, address)?;
            }
        }
    }
}
