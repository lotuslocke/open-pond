use crate::crypto::{AuthKey, CryptoError};
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use thiserror::Error;

pub const MIN_PACKET_SIZE: usize = 72;
pub const MAX_PACKET_SIZE: usize = 2048;
pub const MAX_PAYLOAD_SIZE: usize = 1976;

/// Structure to hold Open Pond Protocol messages
#[derive(Clone, Debug)]
pub struct Message {
    /// Application identifier
    pub id: u8,
    /// Protocol flags
    // 0x80 = Mailbox check
    // 0x40 = Signature check
    pub flags: u8,
    /// Parameter to allow for segmentation
    pub index: u8,
    /// Parameter to allow for different message types
    pub class: u8,
    /// Response return port
    pub port: u16,
    /// Length of the message
    pub length: u16,
    /// Message payload
    pub payload: Vec<u8>,
    /// Message signature
    pub signature: Vec<u8>,
}

impl Message {
    /// Function to generate a new Open Pond message
    pub fn new(id: u8, payload: Vec<u8>) -> MessageResult<Message> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(MessageError::PayloadSizeExceeded {
                size: payload.len(),
            });
        }

        Ok(Message {
            id,
            flags: 0,
            index: 0,
            class: 0,
            port: 0,
            length: payload.len() as u16,
            signature: vec![0; 64],
            payload,
        })
    }

    /// Function to generate an Open Pond message from a bytearray
    pub fn from_bytes(bytes: Vec<u8>) -> MessageResult<Message> {
        if bytes.len() > MAX_PACKET_SIZE {
            return Err(MessageError::MaxPacketSizeExceeded { size: bytes.len() });
        } else if bytes.len() < MIN_PACKET_SIZE {
            return Err(MessageError::MinPacketSizeNotMet { size: bytes.len() });
        }

        Ok(Message {
            id: bytes[0],
            flags: bytes[1],
            index: bytes[2],
            class: bytes[3],
            port: BigEndian::read_u16(&bytes[4..6]),
            length: BigEndian::read_u16(&bytes[6..8]),
            signature: bytes[8..72].to_vec(),
            payload: bytes[72..].to_vec(),
        })
    }

    /// Function to generate a bytearray from an Open Pond message
    pub fn as_bytes(&self) -> MessageResult<Vec<u8>> {
        let mut packet = Vec::with_capacity(MAX_PACKET_SIZE);

        packet.push(self.id);
        packet.push(self.flags);
        packet.push(self.index);
        packet.push(self.class);
        packet.write_u16::<BigEndian>(self.port)?;
        packet.write_u16::<BigEndian>(self.length)?;
        packet.extend(self.signature.clone());
        packet.extend(self.payload.clone());

        Ok(packet)
    }

    /// Function to sign payload and fill out authentication signature
    pub fn sign(&mut self) -> MessageResult<()> {
        let key = AuthKey::load()?;
        self.signature = key.sign(&self.payload)?;

        Ok(())
    }

    /// Function to verify authentication signature
    pub fn verify(&self, pubkey: Vec<u8>) -> MessageResult<bool> {
        let validity = AuthKey::verify(&self.payload, self.signature.clone(), pubkey)?;

        Ok(validity)
    }
}

#[derive(Error, Debug)]
/// Errors generated from Open Pond message operations
pub enum MessageError {
    #[error("Minimum packet size (72) not met: {}", size)]
    MinPacketSizeNotMet { size: usize },
    #[error("Maximum packet size (2048) exceeded: {}", size)]
    MaxPacketSizeExceeded { size: usize },
    #[error("Maximum payload size (1976) exceeded: {}", size)]
    PayloadSizeExceeded { size: usize },
    #[error("Failed to write to packet")]
    PacketFailedWrite(#[from] std::io::Error),
    #[error("Unable to apply cryptographic process to message")]
    CryptoError(#[from] CryptoError),
}

// Convenience alias for Message Results
type MessageResult<T> = Result<T, MessageError>;
