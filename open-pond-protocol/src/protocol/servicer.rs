use crate::config::Settings;
use crate::message::Message;
use crate::protocol::ProtocolResult;

use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;

/// Starts the servicer handling threads
pub fn start_servicer(settings: Settings, local_address: String) -> ProtocolResult<()> {
    // Generate servicer for this portal
    let socket = UdpSocket::bind(local_address)?;
    let manager_port = settings.servicer_manager;
    thread::spawn(move || servicer(socket, manager_port));

    // Start manager for servicing requests between applications
    let endpoint_socket = UdpSocket::bind(format!("0.0.0.0:{}", settings.servicer_manager))?;
    servicer_manager(endpoint_socket)?;

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
        thread::spawn(move || response_handler(message, return_address, manager_port));
    }
}

/// These threads keeps track of responses to requests
pub fn response_handler(
    mut message: Message,
    return_address: String,
    manager_port: u16,
) -> ProtocolResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Send message to application servicer manager
    let manager_address = format!("0.0.0.0:{}", manager_port);
    message.port = socket.local_addr()?.port();
    socket.send_to(&message.as_bytes()?, manager_address)?;

    // Receive response back from application and send back to peers
    let mut response = [0; 1024];
    let (len, _) = socket.recv_from(&mut response)?;
    socket.send_to(&response[0..len], return_address)?;
    Ok(())
}

// Process that takes application requests and stores them until the application is ready to service them
fn servicer_manager(socket: UdpSocket) -> ProtocolResult<()> {
    // Avoid stalling waiting for requests that may not come
    socket.set_nonblocking(true)?;

    // Setup mailbox storage for application responses
    let mut mailboxes: HashMap<u8, Vec<Message>> = HashMap::new();

    loop {
        let mut request = [0; 1024];
        if let Ok((len, address)) = socket.recv_from(&mut request) {
            let message = Message::from_bytes(request[0..len].to_vec())?;

            if message.flags >= 0x80 {
                if let Some(mailbox) = mailboxes.get_mut(&message.id) {
                    if !mailbox.is_empty() {
                        let request = mailbox.remove(0);
                        socket.send_to(&request.as_bytes()?, address)?;
                    }
                }
            } else if let Some(mailbox) = mailboxes.get_mut(&message.id) {
                mailbox.push(message);
            } else {
                let mut mailbox = Vec::new();
                let id = message.id;
                mailbox.push(message);
                mailboxes.insert(id, mailbox);
            }
        }
    }
}
