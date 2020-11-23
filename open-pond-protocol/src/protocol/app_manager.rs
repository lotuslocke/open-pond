use crate::message::Message;
use crate::protocol::ProtocolResult;
use crate::queue::MessageQueue;

use byteorder::{BigEndian, ByteOrder};
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
            thread::sleep(time::Duration::new(5, 0));
            println!("outgoing buffer {}: {}", manager.outgoing.id, Arc::strong_count(&manager.outgoing));

            // Read incoming requests from application
            let mut request = [0; 1];
            if let Ok((_, address)) = manager.socket.recv_from(&mut request) {
                match request[0] {
                    1 => manager.write(address)?,
                    2 => manager.read(address)?,
                    3 => manager.request_length(address)?,
                    _ => (),
                };
            }
            println!("A Manager outgoing buffer: {:?}", manager.outgoing);
        }
    }

    // Function to write a message from an application
    fn write(&self, app: SocketAddr) -> ProtocolResult<()> {
        println!("Success receipt");
        let mut payload = [0; 1024];
        let (len, _) = self.socket.recv_from(&mut payload)?;
        let message = Message::new(self.id, payload[0..len].to_vec())?;
        self.outgoing.push(message)?;
        println!("B Manager outgoing buffer: {:?}", self.outgoing);
        println!("outgoing buffer {}: {}", self.outgoing.id, Arc::strong_count(&self.outgoing));

        let length = len as u16;
        let mut response = [0; 2];
        BigEndian::write_u16(&mut response, length);
        self.socket.send_to(&response, app)?;
        Ok(())
    }

    // Function to read a message from an application
    fn read(&self, app: SocketAddr) -> ProtocolResult<()> {
        let message = self.incoming.pop()?;
        self.socket.send_to(&message.payload, app)?;
        Ok(())
    }

    // Function to give application current incoming queue length
    fn request_length(&self, app: SocketAddr) -> ProtocolResult<()> {
        let size = self.incoming.size() as u8;
        let response = [size];
        self.socket.send_to(&response, app)?;
        Ok(())
    }
}
