use crate::message::Message;

use std::collections::HashMap;

/// Processor function to validate and verify incoming messages
pub fn process_external(mailboxes: &mut HashMap<u8, Vec<Message>>, message: Message) {
    // Validate message signature if present
    if (message.flags & 0x40) > 0 {
        // TODO: Need to figure out how to gather peer pubkey
        // message.verify(pubkey);
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
