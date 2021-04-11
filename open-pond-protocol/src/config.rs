use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
/// Structure to hold Open Pond Node configuration
pub struct Config {
    /// Protocol configuration settings
    pub settings: Settings,
    /// Local host attributes
    pub local: Local,
    /// Peer computer attributes
    pub peers: Vec<Peer>,
    /// Application details
    pub apps: Vec<Application>,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to hold Open Pond Protocol settings
pub struct Settings {
    /// Requester write port
    pub requester_write: u16,
    /// Responder read port
    pub responder_read: u16,
    /// Servicer manager port
    pub servicer_manager: u16,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to hold local host information
pub struct Local {
    /// Socket address of node (IP:Port)
    pub address: String,
    /// Human readable identifier
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to hold Peer information
pub struct Peer {
    /// Socket address of node (IP:Port)
    pub address: String,
    /// Human readable identifier
    pub name: String,
    /// Authentication public key
    pub pubkey: Vec<u8>,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to hold application information of Open Pond Nodes
pub struct Application {
    /// Unique identifier of application
    pub id: u8,
    /// Human readable identifier
    pub name: String,
}

/// Parse file to put out configuration data
pub fn parse_config(file: String) -> std::io::Result<Config> {
    let text = fs::read_to_string(file)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}
