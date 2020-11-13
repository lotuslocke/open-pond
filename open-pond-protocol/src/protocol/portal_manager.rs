use crate::message::Message;
use crate::queue::MessageQueue;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time;

pub struct PortalManager {
    pub stream: TcpStream,
    pub incoming: Arc<Vec<Arc<MessageQueue>>>,
    pub outgoing: Arc<MessageQueue>,
}

impl PortalManager {
    pub fn start_portal(mut manager: PortalManager) -> std::io::Result<()> {
        loop {
            thread::sleep(time::Duration::new(1, 0));

            // Read incoming messages if any are waiting
            let mut request = [0; 1024];
            if let Ok(size) = manager.stream.read(&mut request) {
                let message = Message::from_bytes((&request[0..size]).to_vec()).unwrap();
                let id = message.id as usize;
                manager.incoming[id].push(message).unwrap();
            }

            // Send outgoing messages if any are waiting
            if manager.outgoing.size() > 0 {
                let message = manager.outgoing.pop().unwrap();
                manager
                    .stream
                    .write_all(&message.as_bytes().unwrap())
                    .unwrap();
            }
        }
    }
}
