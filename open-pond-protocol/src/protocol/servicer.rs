use crate::config::Application;
use crate::protocol::app_manager::AppManager;
use crate::protocol::portal_manager::PortalManager;
use crate::protocol::ProtocolResult;
use crate::queue::MessageQueue;

use std::net::{TcpListener, UdpSocket};
use std::sync::Arc;
use std::thread;

/// Starts the servicer handling threads
pub fn start_servicer(address: String, apps: Vec<Application>) -> ProtocolResult<()> {
    let server = TcpListener::bind(address)?;
    let outgoing = Arc::new(MessageQueue::new());
    let mut queues: Vec<Arc<MessageQueue>> = Vec::new();

    println!("outgoing buffer {}: {}", outgoing.id, Arc::strong_count(&outgoing));

    for app in apps.iter() {
        let app_mailbox = Arc::new(MessageQueue::new());

        println!("servicer: {}", app.servicer);
        println!("requester: {}", app.requester);
        let manager = AppManager {
            id: app.id,
            socket: UdpSocket::bind(app.requester.clone())?,
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

                println!("A outgoing buffer {}: {}", outgoing.id, Arc::strong_count(&outgoing));
                println!("Accepting peer's connection");
                let to_client = PortalManager {
                    stream,
                    incoming: incoming.clone(),
                    outgoing: outgoing.clone(),
                };
                println!("B outgoing buffer {}: {}", to_client.outgoing.id, Arc::strong_count(&to_client.outgoing));
                thread::spawn(|| PortalManager::start_portal(to_client));
                println!("C outgoing buffer {}: {}", outgoing.id, Arc::strong_count(&outgoing));
            }
            Err(_) => println!("An error occured trying to make connection!"),
        };
    }

    Ok(())
}
