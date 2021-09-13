pub mod extract;
pub mod models;
pub mod observe;
pub mod serve;

use log::info;
use models::Borrower;
use serve::ThreadPool;
use serde_derive::Deserialize;
use std::error::Error;
use std::net::TcpListener;
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
    let (ext_tx, ext_rx): (Sender<Vec<Borrower>>, Receiver<Vec<Borrower>>) = mpsc::channel();
    let _extract_thread = thread::spawn(move || {
        let obs_rx = Arc::new(Mutex::new(obs_rx));
        extract::handle_extract_borrow_data(obs_rx, ext_tx);
    });

    // extract_thread.join().unwrap();

    // Eventually the server will present data from a database,
    // for now we'll read directly from the ext_rx receiver.

    // Start the server thread
    info!("Starting server keepalive thread");
    let (_srv_tx, _srv_rx): (Sender<Box<dyn FnOnce() + Send>>, Receiver<Box<dyn FnOnce() + Send>>) = mpsc::channel();
    let server_thread = thread::spawn(move || {
        let ext_rx = Arc::new(Mutex::new(ext_rx));
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let ext_rx = Arc::clone(&ext_rx);

            pool.execute(|| {
                serve::handle_connection(stream, ext_rx);
            });
        }
    });

    server_thread.join().unwrap();

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
