use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
/// Structure to hold Open Pond Node configuration
pub struct Config {
    /// Address of the servicer
    pub servicer: Address,
    /// Address of peers
    pub peers: Vec<Address>,
}

#[derive(Deserialize, Debug)]
/// Structure to hold address information of Open Pond Nodes
pub struct Address {
    /// Address of node (IP:Port)
    pub address: String,
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
