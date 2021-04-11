use crate::config::Peer;
use crate::message::Message;

use std::collections::HashMap;

/// Processor function to validate and verify incoming requests
pub fn process_request(mailboxes: &mut HashMap<u8, Vec<Message>>, message: Message) {
    // Place message into mailbox
    if let Some(mailbox) = mailboxes.get_mut(&message.id) {
        mailbox.push(message);
    } else {
        let mut mailbox = Vec::new();
        let id = message.id;
        mailbox.push(message);
        mailboxes.insert(id, mailbox);
    }
}

/// Processor function to validate and verify incoming responses
pub fn process_response(mailboxes: &mut HashMap<u8, Vec<Message>>, message: Message, peer: &Peer) {
    // Validate signatures from servicer
    if (message.flags & 0x40) > 0 && message.verify(peer.pubkey.clone()).is_err() {
        return;
    }

    // Place message into mailbox
    if let Some(mailbox) = mailboxes.get_mut(&message.id) {
        mailbox.push(message);
    } else {
        let mut mailbox = Vec::new();
        let id = message.id;
        mailbox.push(message);
        mailboxes.insert(id, mailbox);
    }
}
