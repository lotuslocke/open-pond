use crate::message::Message;
use crate::protocol::ProtocolResult;
use crate::queue::MessageQueue;

use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time;

// Structure to application interface manager
pub struct AppManager {
    // Application ID
    pub id: u8,
    // Socket application writes to
    pub socket: UdpSocket,
    // Reference to application's incoming queue
    pub incoming: Arc<MessageQueue>,
    // Reference to the outgoing queue
    pub outgoing: Arc<MessageQueue>,
}

impl AppManager {
    // Function starts running the application manager
    pub fn run(manager: AppManager) -> ProtocolResult<()> {
        loop {
            thread::sleep(time::Duration::new(1, 0));

            // Read incoming requests from application
            let mut request = [0; 1];
            if let Ok((_, address)) = manager.socket.recv_from(&mut request) {
                match request[0] {
                    1 => AppManager::read(&manager, address)?,
                    2 => AppManager::write(&manager, address)?,
                    3 => AppManager::request_length(&manager, address)?,
                    _ => (),
                };
            }
        }
    }

    // Function to read a message from an application
    fn read(manager: &AppManager, app: SocketAddr) -> ProtocolResult<()> {
        let message = manager.incoming.pop()?;
        manager.socket.send_to(&message.payload, app)?;
        Ok(())
    }

    // Function to write a message from an application
    fn write(manager: &AppManager, app: SocketAddr) -> ProtocolResult<()> {
        let mut payload = [0; 1024];
        let (len, _) = manager.socket.recv_from(&mut payload)?;
        let message = Message::new(manager.id, payload[0..len].to_vec())?;
        manager.outgoing.push(message)?;

        let response = [len as u8];
        manager.socket.send_to(&response, app)?;
        Ok(())
    }

    // Function to give application current incoming queue length
    fn request_length(manager: &AppManager, app: SocketAddr) -> ProtocolResult<()> {
        let size = manager.incoming.size() as u8;
        let response = [size];
        manager.socket.send_to(&response, app)?;
        Ok(())
    }
}
