pub mod extract;
pub mod models;
pub mod observe;

use log::info;
use models::Borrower;
use serde_derive::Deserialize;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    observer_uri: String,
    server_address: String,
    server_port: u16,
}

impl Config {
    pub fn new(filepath: &str) -> Result<Config, Box<dyn Error>> {
        let json = fs::read_to_string(filepath).unwrap();
        serde_json::from_str(json.as_str()).map_err(|e| e.into())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let obs_uri = config.observer_uri;

    // Observer websocket keepalive thread
    info!("Starting observer keepalive thread");
    let (obs_tx, obs_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let _observer_thread = thread::spawn(move || {
        observe::handle_new_block_stream(obs_uri, obs_tx);
    });

    // Borrow information extraction keepalive thread
    info!("Starting Borrower extraction keepalive thread");
    let (ext_tx, _ext_rx): (Sender<Vec<Borrower>>, Receiver<Vec<Borrower>>) = mpsc::channel();
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
        let actual = Config::new("config.json").unwrap();

        assert_eq!(expected, actual);
    }

    // Need more tests but my lack of error handling knowledge in Rust is slowing me down.
    // Onward!
}
