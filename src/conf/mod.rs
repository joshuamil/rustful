extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    port: String,
}

pub fn parse_cofig() -> String {

    // Read configuration file and parse TOML into Config
    let path = Path::new("./src/conf.toml");
    let mut file = File::open(&path).expect("Error opening File");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading contents from File");
    let config_contents: &str = contents.as_str();
    let decoded: Config = toml::from_str(config_contents).unwrap();

    // Use config values
    let address = &format!("{}:{}", decoded.address.to_string(),decoded.port.to_string());

    address.to_string()

}
