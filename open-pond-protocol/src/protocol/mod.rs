use crate::message::MessageError;

pub mod requester;
pub mod servicer;

use thiserror::Error;

#[derive(Error, Debug)]
/// Errors generated from Open Pond Protocol threads
pub enum ProtocolError {
    #[error("Unable to serialize or deserialize messages")]
    MessageSerialization(#[from] MessageError),
    #[error("Failed to read or write data")]
    IO(#[from] std::io::Error),
}

/// Convenience alias for Results from protocol threads
pub type ProtocolResult<T> = Result<T, ProtocolError>;
