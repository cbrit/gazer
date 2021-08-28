use std::net::TcpStream;
use tungstenite::{client, WebSocket};
use tungstenite::handshake::server::Response;
use tungstenite::protocol::Message;
use tungstenite::stream::MaybeTlsStream;

pub const OBSERVER_ADDRESS: &str = "ws://observer.terra.dev";
pub const NEW_BLOCK_SUBSCRIPTION: &str = "{ \"subscribe\": \"new_block\", \"chain_id\": \"columbus-4\" }";

pub trait Subscribe {
    fn subscribe(&mut self);
}

impl Subscribe for WebSocket<MaybeTlsStream<TcpStream>> {
    fn subscribe(&mut self) {
        &self.write_message(Message::Text(NEW_BLOCK_SUBSCRIPTION.to_string())).unwrap();
    }
}

pub fn get_connection() -> (WebSocket<MaybeTlsStream<TcpStream>>, Response) {
    client::connect(OBSERVER_ADDRESS).unwrap()
}

pub fn receive() -> Option<String> {
    let mut websocket = get_connection().0;
    websocket.subscribe();

    let msg = websocket.read_message().unwrap();
    println!("{:?}", msg);

    if let Message::Text(m) = msg {
        return Some(m);
    }

    None
}