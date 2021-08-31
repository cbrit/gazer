use log::{debug, error, info, warn};
use std::any::Any;
use std::net::TcpStream;
use std::sync::mpsc::{Sender, SendError};
use std::thread;
use std::thread::JoinHandle;
use tungstenite::WebSocket;
use tungstenite::handshake::server::Response;
use tungstenite::protocol::Message;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

pub const OBSERVER_ADDRESS: &str = "ws://observer.terra.dev";
pub const NEW_BLOCK_SUBSCRIPTION: &str = "{ \"subscribe\": \"new_block\", \"chain_id\": \"columbus-4\" }";

pub trait Subscribe {
    fn subscribe(&mut self, uri: String) -> Result<(), tungstenite::error::Error>;
    fn relay_messages(&mut self, sender: &Sender<String>) -> Result<(), SendError<String>>;
}

impl Subscribe for WebSocket<MaybeTlsStream<TcpStream>> {
    fn subscribe(&mut self, uri: String) -> Result<(), tungstenite::error::Error> {
        let url = Url::parse(uri.as_str()).unwrap();
        self.write_message(Message::Text(NEW_BLOCK_SUBSCRIPTION.to_string()))
    }
    
    fn relay_messages(&mut self, sender: &Sender<String>) -> Result<(), SendError<String>> {
        let msg = self.read_message().unwrap();
        let msg: String = msg.to_string();
        
        if msg.len() == 0 {
            debug!("Not relaying message as it was empty.");
            Ok(())
        }
        else {
            debug!("Got message. Relaying: ");
            if msg.len() >= 20 {
                debug!("{}... [preview]", &msg[..19]);
            }

            sender.send(msg)
        }
    }
}

pub fn get_connection(uri: &str) -> (WebSocket<MaybeTlsStream<TcpStream>>, Response) {
    let url = Url::parse(uri).unwrap();
    tungstenite::connect(url).unwrap()
}

pub fn handle_new_block_stream(uri: String, tx: Sender<String>) {
    // Keepalive loop
    loop {
        info!("Starting new Observer connection thread");

        let sender = tx.clone();
        let uri = uri.clone();
        let observer_thread: JoinHandle<Result<(), tungstenite::error::Error>> = thread::spawn(move || {
            info!("Connecting to Observer...");
            let (mut websocket, _response) = get_connection(uri.as_str());

            info!("Subscribing to new_block stream...");
            match websocket.subscribe(uri) {
                Err(err) => return Err(err),
                _ => (),
            };

            // Listen for new_block messages and relay them over the channel
            info!("Begin relaying new_block messages");
            loop {
                websocket.relay_messages(&sender);
                info!("Data written to channel");
            }
        });

        match observer_thread.join() {
            Err(err) => {
                match err.as_ref().downcast_ref::<String>() {
                    Some(err) => error!("Connection to Observer closed: {}", err),
                    None => warn!("Unable to downcast Error from Observer thread failure"),
                };
            },
            _ => (),
        }
        
        info!("Attempting to re-open Observer connection");
    }
}