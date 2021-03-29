use open_pond_protocol::{AuthKey, CryptoError, Message, MessageError, Settings};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use thiserror::Error;

/// Networking components for Request Endpoint
pub struct RequestEndpoint {
    // Request endpoint
    request_endpoint: UdpSocket,
    // Request write port
    request_write: u16,
    // Application ID
    app_id: u8,
}

/// Networking components for Response Endpoint
pub struct ResponseEndpoint {
    // Response endpoint
    response_endpoint: UdpSocket,
    // Response read port
    response_read: u16,
    // Application ID
    app_id: u8,
}

/// Networking components for Service Endpoint
pub struct ServiceEndpoint {
    // Service endpoint
    service_endpoint: UdpSocket,
    // Servicer manager port
    servicer_manager: u16,
    // Application ID
    app_id: u8,
}

/// Create a new interface objects to interface with the protocol
pub fn new_interface(
    settings: Settings,
    app_id: u8,
) -> APIResult<(RequestEndpoint, ServiceEndpoint, ResponseEndpoint)> {
    let request_socket = UdpSocket::bind("0.0.0.0:0")?;
    let request_endpoint = RequestEndpoint {
        request_endpoint: request_socket,
        request_write: settings.requester_write,
        app_id,
    };

    let service_socket = UdpSocket::bind("0.0.0.0:0")?;
    service_socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    let service_endpoint = ServiceEndpoint {
        service_endpoint: service_socket,
        servicer_manager: settings.servicer_manager,
        app_id,
    };

    let response_socket = UdpSocket::bind("0.0.0.0:0")?;
    response_socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    let response_endpoint = ResponseEndpoint {
        response_endpoint: response_socket,
        response_read: settings.responder_read,
        app_id,
    };

    Ok((request_endpoint, service_endpoint, response_endpoint))
}

/// Get user's public key
pub fn get_public_key() -> APIResult<Vec<u8>> {
    let keypair = AuthKey::load()?;
    Ok(keypair.get_public())
}

impl RequestEndpoint {
    /// Write request to requester
    pub fn write_request(&self, data: Vec<u8>) -> APIResult<()> {
        let message = Message::new(self.app_id, data)?;
        self.request_endpoint.send_to(
            &message.as_bytes()?,
            format!("0.0.0.0:{}", self.request_write),
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

impl ServiceEndpoint {
    /// Read request from the servicer
    pub fn read_request(&self) -> APIResult<(Vec<u8>, SocketAddr)> {
        let mut message = Message::new(self.app_id, Vec::new())?;
        message.flags = 0x80;
        let mut data = [0; 1024];

        loop {
            self.service_endpoint.send_to(
                &message.as_bytes()?,
                format!("0.0.0.0:{}", self.servicer_manager),
            )?;
            if let Ok((len, mut address)) = self.service_endpoint.recv_from(&mut data) {
                let message = Message::from_bytes(data[0..len].to_vec())?;
                address.set_port(message.port);
                return Ok((message.payload, address));
            }
        }
    }

    /// Write response back to requesting peer
    pub fn write_response(&self, return_address: SocketAddr, data: Vec<u8>) -> APIResult<()> {
        let message = Message::new(self.app_id, data)?;
        self.service_endpoint
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
    #[error("Failure with loading public key")]
    PublicKey(#[from] CryptoError),
}

// Convenience alias for API results
type APIResult<T> = Result<T, APIError>;
