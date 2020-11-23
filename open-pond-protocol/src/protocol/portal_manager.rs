use crate::message::Message;
use crate::protocol::ProtocolResult;
use crate::queue::MessageQueue;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time;

// Structure to portal interface manager
pub struct PortalManager {
    // TCP connection between nodes
    pub stream: TcpStream,
    // Reference to all application incoming queues
    pub incoming: Arc<Vec<Arc<MessageQueue>>>,
    // Reference to the outgoing queue
    pub outgoing: Arc<MessageQueue>,
}

impl PortalManager {
    // Function that runs the thread managing the portal between nodes
    pub fn start_portal(mut manager: PortalManager) -> ProtocolResult<()> {
        println!("Portal outgoing buffer {}: {:?}", manager.outgoing.id, manager.outgoing);
        loop {
            thread::sleep(time::Duration::new(5, 0));

            // Read incoming messages if any are waiting
            let mut request = [0; 1024];
            if let Ok(size) = manager.stream.read(&mut request) {
                println!("Shouldn't work");
                let message = Message::from_bytes((&request[0..size]).to_vec())?;
                let id = message.id as usize;
                manager.incoming[id].push(message)?;
            }

            // Send outgoing messages if any are waiting
            if manager.outgoing.size() > 0 {
                println!("Sending message away...");
                let message = manager.outgoing.pop()?;
                manager.stream.write_all(&message.as_bytes()?)?;
            }
        }
    }
}
