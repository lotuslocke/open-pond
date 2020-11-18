use byteorder::{BigEndian, ByteOrder};
use std::net::UdpSocket;
use thiserror::Error;

/// Structure holding APISocket networking components
pub struct APISocket {
    // APISocket's UDP socket
    socket: UdpSocket,
    // Application manager access port
    access_port: String,
}

impl APISocket {
    /// Create a new API socket to read or write to
    pub fn new(access_port: String) -> APIResult<APISocket> {
        Ok(APISocket {
            socket: UdpSocket::bind("0.0.0.0:0")?,
            access_port,
        })
    }

    /// Write data to the application manager
    pub fn opp_write(&self, data: Vec<u8>) -> APIResult<u16> {
        let mut length = [0; 2];
        self.socket.send_to(&[1], self.access_port.clone())?;
        self.socket.send_to(&data, self.access_port.clone())?;
        self.socket.recv_from(&mut length)?;
        Ok(BigEndian::read_u16(&length))
    }

    /// Request data from the application manager
    pub fn opp_read(&self) -> APIResult<Vec<u8>> {
        let mut data = [0; 1024];
        self.socket.send_to(&[2], self.access_port.clone())?;
        let (len, _) = self.socket.recv_from(&mut data)?;
        Ok(data[0..len].to_vec())
    }

    /// Request the number of messages waiting to be read
    /// from the application manager
    pub fn opp_request_length(&self) -> APIResult<u8> {
        let mut length = [0; 1];
        self.socket.send_to(&[3], self.access_port.clone())?;
        self.socket.recv_from(&mut length)?;
        Ok(length[0])
    }
}

#[derive(Error, Debug)]
/// Errors generated from Open Pond API operations
pub enum APIError {
    #[error("Failure with socket operations")]
    SocketIO(#[from] std::io::Error),
}

// Convenience alias for API results
type APIResult<T> = Result<T, APIError>;
