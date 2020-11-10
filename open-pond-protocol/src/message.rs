use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use thiserror::Error;

const MAX_PACKET_SIZE: usize = 1024;
const MAX_PAYLOAD_SIZE: usize = 1020;

// Structure to hold Open Pond Protocol messages
pub struct Message {
    // Unique identifier for application
    pub id: u8,
    // Length of the message
    pub length: u16,
    // Segmentation flag
    pub flag: u8,
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
            length: payload.len() as u16,
            flag: 0,
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
            length: BigEndian::read_u16(&bytes[1..3]),
            flag: bytes[3],
            payload: bytes[4..].to_vec(),
        })
    }

    // Function to generate a bytearray from an Open Pond message
    pub fn as_bytes(&self) -> MessageResult<Vec<u8>> {
        let mut packet = Vec::with_capacity(MAX_PACKET_SIZE);

        packet.push(self.id);
        packet.write_u16::<BigEndian>(self.length)?;
        packet.push(self.flag);
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
