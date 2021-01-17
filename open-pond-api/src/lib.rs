use open_pond_protocol::{Message, MessageError, Settings};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use thiserror::Error;

/// Structure holding interface networking components
pub struct RequesterEndpoint {
    // Request endpoint
    requester_endpoint: UdpSocket,
    // Requester write port
    requester_write: u16,
    // Application ID
    app_id: u8,
}

/// Structure holding interface networking components
pub struct ResponseEndpoint {
    // Response endpoint
    response_endpoint: UdpSocket,
    // Response read port
    response_read: u16,
    // Application ID
    app_id: u8,
}

/// Structure holding interface networking components
pub struct ServicerEndpoint {
    // Servicer endpoint
    servicer_endpoint: UdpSocket,
    // Servicer manager port
    servicer_manager: u16,
    // Application ID
    app_id: u8,
}

/// Create a new interface objects to interface with the protocol
pub fn new_interface(
    settings: Settings,
    app_id: u8,
) -> APIResult<(RequesterEndpoint, ServicerEndpoint, ResponseEndpoint)> {
    let requester_socket = UdpSocket::bind("0.0.0.0:0")?;
    let requester_endpoint = RequesterEndpoint {
        requester_endpoint: requester_socket,
        requester_write: settings.requester_write,
        app_id,
    };

    let servicer_socket = UdpSocket::bind("0.0.0.0:0")?;
    servicer_socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    let servicer_endpoint = ServicerEndpoint {
        servicer_endpoint: servicer_socket,
        servicer_manager: settings.servicer_manager,
        app_id,
    };

    let response_socket = UdpSocket::bind("0.0.0.0:0")?;
    response_socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    let response_endpoint = ResponseEndpoint {
        response_endpoint: response_socket,
        response_read: settings.requester_read,
        app_id,
    };

    Ok((requester_endpoint, servicer_endpoint, response_endpoint))
}

impl RequesterEndpoint {
    /// Write request to requester
    pub fn write_request(&self, data: Vec<u8>) -> APIResult<()> {
        let message = Message::new(self.app_id, data)?;
        self.requester_endpoint.send_to(
            &message.as_bytes()?,
            format!("0.0.0.0:{}", self.requester_write),
        )?;
        Ok(())
    }
}

impl ResponseEndpoint {
    /// Read response from requester mailbox
    pub fn read_response(&self) -> APIResult<Vec<u8>> {
        let mut message = Message::new(self.app_id, Vec::new())?;
        message.flags = 0x80;
        let mut data = [0; 1024];

        loop {
            self.response_endpoint.send_to(
                &message.as_bytes()?,
                format!("0.0.0.0:{}", self.response_read),
            )?;

            if let Ok((len, _)) = self.response_endpoint.recv_from(&mut data) {
                let message = Message::from_bytes(data[0..len].to_vec())?;
                return Ok(message.payload);
            }
        }
    }
}

impl ServicerEndpoint {
    /// Read request from the servicer
    pub fn read_request(&self) -> APIResult<(Vec<u8>, SocketAddr)> {
        let mut message = Message::new(self.app_id, Vec::new())?;
        message.flags = 0x80;
        let mut data = [0; 1024];

        loop {
            self.servicer_endpoint.send_to(
                &message.as_bytes()?,
                format!("0.0.0.0:{}", self.servicer_manager),
            )?;
            if let Ok((len, mut address)) = self.servicer_endpoint.recv_from(&mut data) {
                let message = Message::from_bytes(data[0..len].to_vec())?;
                address.set_port(message.port);
                return Ok((message.payload, address));
            }
        }
    }

    /// Write response back to requester
    pub fn write_response(&self, return_address: SocketAddr, data: Vec<u8>) -> APIResult<()> {
        let message = Message::new(self.app_id, data)?;
        self.servicer_endpoint
            .send_to(&message.as_bytes()?, return_address)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
/// Errors generated from Open Pond API operations
pub enum APIError {
    #[error("Failure with socket operations")]
    SocketIO(#[from] std::io::Error),
    #[error("Failure with message formatting")]
    InvalidMessage(#[from] MessageError),
}

// Convenience alias for API results
type APIResult<T> = Result<T, APIError>;
