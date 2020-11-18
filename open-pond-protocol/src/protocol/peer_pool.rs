use crate::config::{Address, Application};
use crate::protocol::app_manager::AppManager;
use crate::protocol::portal_manager::PortalManager;
use crate::protocol::ProtocolResult;
use crate::queue::MessageQueue;

use std::iter;
use std::net::{TcpStream, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time;

/// Spawns request threads for each peer in the active peer pool
pub fn start_peer_pool(peers: Vec<Address>, apps: Vec<Application>) -> ProtocolResult<()> {
    let mut active: Vec<bool> = iter::repeat(false).take(peers.len()).collect();

    let outgoing = Arc::new(MessageQueue::new());
    let mut queues: Vec<Arc<MessageQueue>> = Vec::new();

    for app in apps.iter() {
        let app_mailbox = Arc::new(MessageQueue::new());

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
    loop {
        thread::sleep(time::Duration::new(1, 0));
        for (i, peer) in peers.iter().enumerate() {
            if !active[i] {
                if let Ok(stream) = TcpStream::connect(peer.address.clone()) {
                    println!("{} accepted connection request", peer.name.clone());
                    let to_peer = PortalManager {
                        stream,
                        incoming: incoming.clone(),
                        outgoing: outgoing.clone(),
                    };
                    thread::spawn(|| PortalManager::start_portal(to_peer));
                    active[i] = true;
                }
            }
        }
    }
}
