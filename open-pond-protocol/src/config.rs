use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
/// Structure to hold Open Pond Node configuration
pub struct Config {
    /// Protocol configuration settings
    pub settings: Settings,
    /// Address of the local computer
    pub local: Address,
    /// Addresses of the peer computers
    pub peers: Vec<Address>,
    /// Applications detail
    pub apps: Vec<Application>,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to Open Pond Protocol settings
pub struct Settings {
    /// Requester write port
    pub requester_write: u16,
    /// Requester read port
    pub requester_read: u16,
    /// Servicer read port
    pub servicer: u16,
    /// Servicer manager port
    pub servicer_manager: u16,
}

#[derive(Deserialize, Debug, Clone)]
/// Structure to hold address information of Open Pond Nodes
pub struct Address {
    /// Address of node (IP:Port)
    pub address: String,
    /// Human readable identifier
    pub name: String,
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
    let mut text = String::new();
    File::open(file)?.read_to_string(&mut text)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}
