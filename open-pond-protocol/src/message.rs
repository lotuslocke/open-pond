use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use thiserror::Error;

const MAX_PACKET_SIZE: usize = 1024;
const MAX_PAYLOAD_SIZE: usize = 1018;

// Structure to hold Open Pond Protocol messages
#[derive(Clone, Debug)]
pub struct Message {
    // Unique identifier for application
    pub id: u8,
    // Protocol flags
    // 0x80 = Internal request
    pub flags: u8,
    // Response return port
    pub port: u16,
    // Length of the message
    pub length: u16,
    // Message payload
    pub payload: Vec<u8>,
}

impl Message {
    // Function to generate a new Open Pond message
    pub fn new(id: u8, payload: Vec<u8>) -> MessageResult<Message> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(MessageError::PayloadSizeExceeded {
                size: payload.len(),
            });
        }

        Ok(Message {
            id,
            flags: 0,
            port: 0,
            length: payload.len() as u16,
            payload,
        })
    }

    // Function to generate an Open Pond message from a bytearray
    pub fn from_bytes(bytes: Vec<u8>) -> MessageResult<Message> {
        if bytes.len() > MAX_PACKET_SIZE {
            return Err(MessageError::MaxPacketSizeExceeded { size: bytes.len() });
        }

        Ok(Message {
            id: bytes[0],
            flags: bytes[1],
            port: BigEndian::read_u16(&bytes[2..4]),
            length: BigEndian::read_u16(&bytes[4..6]),
            payload: bytes[6..].to_vec(),
        })
    }

    // Function to generate a bytearray from an Open Pond message
    pub fn as_bytes(&self) -> MessageResult<Vec<u8>> {
        let mut packet = Vec::with_capacity(MAX_PACKET_SIZE);

        packet.push(self.id);
        packet.push(self.flags);
        packet.write_u16::<BigEndian>(self.port)?;
        packet.write_u16::<BigEndian>(self.length)?;
        packet.extend(self.payload.clone());

        Ok(packet)
    }
}

#[derive(Error, Debug)]
// Errors generated from Open Pond message operations
pub enum MessageError {
    #[error("Maximum packet size exceeded: {}", size)]
    MaxPacketSizeExceeded { size: usize },
    #[error("Maximum payload size exceeded: {}", size)]
    PayloadSizeExceeded { size: usize },
    #[error("Failed to write to packet")]
    PacketFailedWrite(#[from] std::io::Error),
}

// Convenience alias for Message results
type MessageResult<T> = Result<T, MessageError>;
