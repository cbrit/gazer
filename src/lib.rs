pub mod extract;
pub mod observe;
pub mod models;

use log::{debug, error, log_enabled, info, Level};
use models::{Borrower};
use serde_derive::Deserialize;
use std::{fs, thread};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

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
    let (obs_uri, addr, port) = (config.observer_uri, config.server_address, config.server_port);
    
    // Observer websocket keepalive thread
    info!("Starting observer keepalive thread");
    let (obs_tx, obs_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let observer_thread = thread::spawn(move || {
        observe::handle_new_block_stream(obs_uri, obs_tx);
    });

    // Borrow information extraction keepalive thread
    info!("Starting Borrower extraction keepalive thread");
    let (ext_tx, ext_rx): (Sender<Vec<Borrower>>, Receiver<Vec<Borrower>>) = mpsc::channel();
    let extract_thread = thread::spawn(move || {
        let obs_rx = Arc::new(Mutex::new(obs_rx));
        extract::handle_extract_borrow_data(obs_rx, ext_tx);
    });

    extract_thread.join().unwrap();

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