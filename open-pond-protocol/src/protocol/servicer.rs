use crate::config::Application;
use crate::protocol::app_manager::AppManager;
use crate::protocol::portal_manager::PortalManager;
use crate::queue::MessageQueue;

use std::net::{TcpListener, UdpSocket};
use std::sync::Arc;
use std::thread;

/// Starts the servicer handling threads
pub fn start_servicer(address: String, apps: Vec<Application>) -> std::io::Result<()> {
    let server = TcpListener::bind(address)?;
    let outgoing = Arc::new(MessageQueue::new());
    let mut queues: Vec<Arc<MessageQueue>> = Vec::new();

    for app in apps.iter() {
        let app_mailbox = Arc::new(MessageQueue::new());

        let manager = AppManager {
            id: app.id,
            socket: UdpSocket::bind(app.servicer.clone()).unwrap(),
            incoming: app_mailbox.clone(),
            outgoing: outgoing.clone(),
        };

        thread::spawn(|| AppManager::run(manager));
        queues.push(app_mailbox);
    }

    let incoming = Arc::new(queues);
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepting peer's connection");
                let to_client = PortalManager {
                    stream,
                    incoming: incoming.clone(),
                    outgoing: outgoing.clone(),
                };
                thread::spawn(|| PortalManager::start_portal(to_client));
            }
            Err(_) => println!("An error occured trying to make connection!"),
        };
    }

    Ok(())
}
