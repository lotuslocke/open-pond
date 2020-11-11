use crate::app_manager::AppManager;
use crate::config::Application;
use crate::message::Message;
use crate::queue::MessageQueue;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time;

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

        AppManager::run(manager);
        queues.push(app_mailbox);
    }

    let incoming = Arc::new(queues);
    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepting peer's connection");
                let client = ServicerThread {
                    stream,
                    incoming: incoming.clone(),
                    outgoing: outgoing.clone(),
                };
                thread::spawn(|| ServicerThread::serve(client));
            }
            Err(_) => println!("An error occured trying to make connection!"),
        };
    }

    Ok(())
}

struct ServicerThread {
    stream: TcpStream,
    incoming: Arc<Vec<Arc<MessageQueue>>>,
    outgoing: Arc<MessageQueue>,
}

impl ServicerThread {
    fn serve(mut thread: ServicerThread) -> std::io::Result<()> {
        loop {
            thread::sleep(time::Duration::new(1, 0));

            // Read incoming requests if any are waiting
            let mut request = [0; 1024];
            if let Ok(size) = thread.stream.read(&mut request) {
                let message = Message::from_bytes((&request[0..size]).to_vec()).unwrap();
                let id = message.id as usize;
                thread.incoming[id].push(message).unwrap();
            }

            // Send outgoing responses if any are waiting
            if thread.outgoing.size() > 0 {
                let message = thread.outgoing.pop().unwrap();
                thread
                    .stream
                    .write_all(&message.as_bytes().unwrap())
                    .unwrap();
            }
        }
    }
}
