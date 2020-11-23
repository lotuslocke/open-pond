use crate::message::Message;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use rand::prelude::*;

// Thread safe structure used to hold messages
#[derive(Debug)]
pub struct MessageQueue {
    // Thread safe message queue
    pub queue: Mutex<Vec<Message>>,
    // Size of message queue
    pub size: AtomicUsize,
    pub id: u8,
}

impl MessageQueue {
    // Function to generate a new Open Pond message queue
    pub fn new() -> MessageQueue {
        MessageQueue {
            queue: Mutex::new(Vec::new()),
            size: AtomicUsize::new(0),
            id: random(),
        }
    }

    // Function that puts a message on the Open Pond message queue
    pub fn size(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    // Function that puts a message on the Open Pond message queue
    pub fn push(&self, message: Message) -> QueueResult<()> {
        let mut messages = match self.queue.lock() {
            Err(_) => return Err(QueueError::MutexPoisoned),
            Ok(messages) => messages,
        };

        messages.push(message);
        let size = self.size.load(Ordering::Acquire);
        self.size.store(size + 1, Ordering::Release);
        Ok(())
    }

    // Function that takes a message from the Open Pond message queue
    pub fn pop(&self) -> QueueResult<Message> {
        let mut messages = match self.queue.lock() {
            Err(_) => return Err(QueueError::MutexPoisoned),
            Ok(messages) => messages,
        };

        let size = self.size.load(Ordering::Acquire);
        if size == 0 {
            return Err(QueueError::EmptyQueue);
        }

        let message = messages.remove(0);
        self.size.store(size + 1, Ordering::Release);
        Ok(message)
    }
}

#[derive(Error, Debug)]
// Errors generated from Open Pond message queue operations
pub enum QueueError {
    #[error("Attempted to pop message off empty MessageQueue")]
    EmptyQueue,
    #[error("Attempted to access queue whose mutex was posioned")]
    MutexPoisoned,
}

// Convenience alias for MessageQueue results
type QueueResult<T> = Result<T, QueueError>;
