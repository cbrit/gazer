pub mod extract;
pub mod observe;
pub mod models;

use models::{Borrower};
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Deserialize, PartialEq)]
pub struct Config {
    observer_uri: String,
    server_address: String,
    server_port: u16,
}

impl Config {
    pub fn new(filepath: &str) -> Result<Config, Box<dyn Error>> {
        let json = fs::read_to_string(filepath).unwrap();
        serde_json::from_str(json.as_str())
            .map_err(|e| e.into())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let json = fs::read_to_string("response.json").unwrap();
    let json = match observe::receive() {
        Some(j) => j,
        None => panic!("Didn't receive any json output"),
    };
    let txs = extract::get_transactions(json).unwrap();
    let borrow_events = extract::get_borrow_events(&txs).unwrap();
    let borrowers: Vec<Borrower> = borrow_events.into_iter().map(Borrower::new).collect();

    println!("{:?}", borrowers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_config() {
        let expected = Config {
            observer_uri: "wss://observer.terra.dev/".to_string(),
            server_address: "localhost".to_string(),
            server_port: 7878,
        };
        let actual = Config::new("config.json");
    }

    // Need more tests but my lack of error handling knowledge in Rust is slowing me down.
    // Onward!
}