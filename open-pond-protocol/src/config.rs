use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
/// Structure to hold Open Pond Node configuration
pub struct Config {
    /// Address of the servicer
    pub servicer: Address,
    /// Peer addresses
    pub peers: Vec<Address>,
    /// Applications
    pub apps: Vec<Application>,
}

#[derive(Deserialize, Debug)]
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
    /// Requester application endpoint
    pub requester: String,
    /// Servicer application endpoint
    pub servicer: String,
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
