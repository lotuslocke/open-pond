use crate::message::Message;
use crate::queue::MessageQueue;

use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time;

pub struct AppManager {
    pub id: u8,
    pub socket: UdpSocket,
    pub incoming: Arc<MessageQueue>,
    pub outgoing: Arc<MessageQueue>,
}

impl AppManager {
    // Function starts running the application manager
    pub fn run(manager: AppManager) {
        loop {
            thread::sleep(time::Duration::new(1, 0));

            // Read incoming requests from application
            let mut request = [0; 1];
            if let Ok((_, address)) = manager.socket.recv_from(&mut request) {
                match request[0] {
                    1 => AppManager::read(&manager, address),
                    2 => AppManager::write(&manager, address),
                    3 => AppManager::request_length(&manager, address),
                    _ => (),
                };
            }
        }
    }

    fn read(manager: &AppManager, app: SocketAddr) {
        let message = manager.incoming.pop().unwrap();
        manager.socket.send_to(&message.payload, app).unwrap();
    }

    fn write(manager: &AppManager, app: SocketAddr) {
        let mut payload = [0; 1024];
        let (len, _) = manager.socket.recv_from(&mut payload).unwrap();
        let message = Message::new(manager.id, payload[0..len].to_vec()).unwrap();
        manager.outgoing.push(message).unwrap();
        let response = [len as u8];
        manager.socket.send_to(&response, app).unwrap();
    }

    fn request_length(manager: &AppManager, app: SocketAddr) {
        let size = manager.incoming.size() as u8;
        let response = [size];
        manager.socket.send_to(&response, app).unwrap();
    }
}
