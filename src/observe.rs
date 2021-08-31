use std::net::TcpStream;
use std::sync::mpsc::{Sender};
use std::thread;
use tungstenite::WebSocket;
use tungstenite::handshake::server::Response;
use tungstenite::protocol::Message;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

pub const OBSERVER_ADDRESS: &str = "ws://observer.terra.dev";
pub const NEW_BLOCK_SUBSCRIPTION: &str = "{ \"subscribe\": \"new_block\", \"chain_id\": \"columbus-4\" }";

pub trait Subscribe {
    fn subscribe(&mut self, uri: String);
    fn relay_messages(&mut self, sender: &Sender<String>);
}

impl Subscribe for WebSocket<MaybeTlsStream<TcpStream>> {
    fn subscribe(&mut self, uri: String) {
        let url = Url::parse(uri.as_str()).unwrap();
        self.write_message(Message::Text(NEW_BLOCK_SUBSCRIPTION.to_string())).unwrap();
    }
    
    fn relay_messages(&mut self, sender: &Sender<String>) {
        let msg = self.read_message().unwrap();
        let msg: String = msg.to_string();
        sender.send(msg).unwrap();
    }
}

pub fn get_connection(uri: &str) -> (WebSocket<MaybeTlsStream<TcpStream>>, Response) {
    let url = Url::parse(uri).unwrap();
    tungstenite::connect(url).unwrap()
}

pub fn handle_new_block_stream(uri: String, tx: Sender<String>) {
    // Keepalive loop
    loop {
        println!("Observer: Spawning new thread");

        let sender = tx.clone();
        let uri = uri.clone();
        let observer_thread = thread::spawn(move || {
            println!("Connecting to observer...");
            let (mut websocket, _response) = get_connection(uri.as_str());

            println!("Subscribing...");
            websocket.subscribe(uri);
            
            // Listen for new_block messages and relay them over the channel
            println!("Begin relaying");
            loop {
                websocket.relay_messages(&sender);
            }
        });

        observer_thread.join().unwrap();
    }
}