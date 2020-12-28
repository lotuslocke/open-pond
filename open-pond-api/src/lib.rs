use open_pond_protocol::Settings;
use std::net::{SocketAddr, UdpSocket};
use thiserror::Error;

/// Structure holding interface networking components
pub struct RequesterEndpoint {
    // Request endpoint
    requester_endpoint: UdpSocket,
    // Requester write port
    requester_write: u16,
    // Requester read port
    requester_read: u16,
    // Application ID
    app_id: u8,
}

/// Structure holding interface networking components
pub struct ServicerEndpoint {
    // Servicer endpoint
    servicer_endpoint: UdpSocket,
    //
    servicer_manager: u16,
    // Application ID
    app_id: u8,
}

/// Create a new interface objects to interface with the protocol
pub fn new_interface(
    settings: Settings,
    app_id: u8,
) -> APIResult<(RequesterEndpoint, ServicerEndpoint)> {
    let requester_endpoint = RequesterEndpoint {
        requester_endpoint: UdpSocket::bind("0.0.0.0:0")?,
        requester_write: settings.requester_write,
        requester_read: settings.requester_read,
        app_id,
    };

    let servicer_endpoint = ServicerEndpoint {
        servicer_endpoint: UdpSocket::bind("0.0.0.0:0")?,
        servicer_manager: settings.servicer_manager,
        app_id,
    };

    Ok((requester_endpoint, servicer_endpoint))
}

impl RequesterEndpoint {
    /// Write request to requester
    pub fn write_request(&self, mut data: Vec<u8>) -> APIResult<()> {
        data.insert(0, self.app_id);
        self.requester_endpoint
            .send_to(&data, format!("0.0.0.0:{}", self.requester_write))?;
        Ok(())
    }

    /// Read response from requester mailbox
    pub fn read_response(&self) -> APIResult<Vec<u8>> {
        self.requester_endpoint
            .send_to(&[self.app_id], format!("0.0.0.0:{}", self.requester_read))?;
        let mut data = [0; 1018];
        let (len, _) = self.requester_endpoint.recv_from(&mut data)?;
        Ok(data[0..len].to_vec())
    }
}

impl ServicerEndpoint {
    /// Read request from the servicer
    pub fn read_request(&self) -> APIResult<(Vec<u8>, SocketAddr)> {
        self.servicer_endpoint
            .send_to(&[self.app_id], format!("0.0.0.0:{}", self.servicer_manager))?;
        let mut data = [0; 1018];
        let (len, address) = self.servicer_endpoint.recv_from(&mut data)?;
        Ok((data[0..len].to_vec(), address))
    }

    /// Write response back to requester
    pub fn write_response(&self, return_address: SocketAddr, data: Vec<u8>) -> APIResult<()> {
        self.servicer_endpoint.send_to(&data, return_address)?;
        Ok(())
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
