use crate::app_manager::AppManager;
use crate::config::{Address, Application};
use crate::message::Message;
use crate::queue::MessageQueue;

use std::io::{Read, Write};
use std::iter;
use std::net::{TcpStream, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time;

/// Spawns request threads for each peer in the active peer pool
pub fn start_peer_pool(peers: Vec<Address>, apps: Vec<Application>) -> std::io::Result<()> {
    let mut active: Vec<bool> = iter::repeat(false).take(peers.len()).collect();

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
    loop {
        thread::sleep(time::Duration::new(1, 0));
        for (i, peer) in peers.iter().enumerate() {
            if !active[i] {
                if let Ok(stream) = TcpStream::connect(peer.address.clone()) {
                    println!("{} accepted connection request", peer.name.clone());
                    let peer = PoolThread {
                        stream,
                        incoming: incoming.clone(),
                        outgoing: outgoing.clone(),
                    };
                    thread::spawn(|| PoolThread::request(peer));
                    active[i] = true;
                }
            }
        }
    }
}

struct PoolThread {
    stream: TcpStream,
    incoming: Arc<Vec<Arc<MessageQueue>>>,
    outgoing: Arc<MessageQueue>,
}

impl PoolThread {
    fn request(mut thread: PoolThread) -> std::io::Result<()> {
        loop {
            thread::sleep(time::Duration::new(1, 0));

            // Read incoming responses if any are waiting
            let mut response = [0; 1024];
            if let Ok(size) = thread.stream.read(&mut response) {
                let message: Message = Message::from_bytes((&response[0..size]).to_vec()).unwrap();
                let id = message.id as usize;
                thread.incoming[id].push(message).unwrap();
            }

            // Send outgoing requests if any are waiting
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
