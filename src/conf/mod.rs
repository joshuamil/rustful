extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


/// Reads a file and returns the contents as a String
fn read_file(loc: &str) -> String {
    let path = Path::new(&loc);
    let mut file = File::open(&path).expect("Error opening File");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading contents from File");
    contents
}

/// Parses a TOML configuration file and returns the server address as a response
pub fn parse_config() -> String {

    #[derive(Debug, Deserialize)]
    struct Config {
        address: String,
        port: String,
    }

    // Read configuration file and parse TOML into Config
    let contents = read_file("./src/conf.toml");
    let config_contents: &str = contents.as_str();
    let decoded: Config = toml::from_str(config_contents).unwrap();

    // Use config values
    let address = &format!("{}:{}", decoded.address.to_string(),decoded.port.to_string());

    address.to_string()
}

/// TODO: Will eventually parse the Endpoints JSON file for configuration options
pub fn parse_endpoints() -> String {

    // Read configuration file and parse JSON into Config
    let contents = read_file("./src/endpoints.json");
    let config_contents: &str = contents.as_str();

    config_contents.to_string()
}
