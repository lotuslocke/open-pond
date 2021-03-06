use crate::config::{Address, Settings};
use crate::message::Message;
use crate::protocol::ProtocolResult;

use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

/// Spawns the threads associated with the client side of the Open Pond Protocol
pub fn start_requester(settings: Settings, peers: Vec<Address>) -> ProtocolResult<()> {
    // Generate request writer for this portal
    let write_socket = UdpSocket::bind(format!("0.0.0.0:{}", settings.requester_write))?;
    let return_port = settings.responder_read;
    thread::spawn(move || peer_writer(write_socket, peers, return_port));

    // Generate response reader for this portal
    let read_socket = UdpSocket::bind(format!("0.0.0.0:{}", settings.responder_read))?;
    thread::spawn(|| peer_reader(read_socket));

    Ok(())
}

// Process that takes incoming messages from applications and writes them to different peers
fn peer_writer(socket: UdpSocket, peers: Vec<Address>, return_port: u16) -> ProtocolResult<()> {
    loop {
        // Receive message from an application
        let mut payload = [0; 1024];
        let (len, _) = socket.recv_from(&mut payload)?;
        let mut message = Message::from_bytes(payload[0..len].to_vec())?;
        message.port = return_port;

        // Broadcast message to network
        for peer in &peers {
            socket.send_to(&message.as_bytes()?, peer.address.clone())?;
        }
    }
}

// Process that takes responses to application requests and stores them until requested by the application
fn peer_reader(socket: UdpSocket) -> ProtocolResult<()> {
    // Avoid stalling waiting for requests that may not come
    socket.set_read_timeout(Some(Duration::from_millis(1)))?;

    // Setup mailbox storage for application responses
    let mut mailboxes: HashMap<u8, Vec<Message>> = HashMap::new();

    loop {
        let mut response = [0; 1024];
        if let Ok((len, address)) = socket.recv_from(&mut response) {
            let message = Message::from_bytes(response[0..len].to_vec())?;

            // If message is a request for response data, get next response
            // from the mailbox if available
            if message.flags >= 0x80 {
                if let Some(mailbox) = mailboxes.get_mut(&message.id) {
                    if !mailbox.is_empty() {
                        let response = mailbox.remove(0);
                        socket.send_to(&response.as_bytes()?, address)?;
                    }
                }

            // If the message is a response, add to mailbox and create mailbox
            // if needed
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
