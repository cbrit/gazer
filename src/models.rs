use serde::Deserialize;
use serde_derive::Deserialize;

// Models for drilling down into the JSON response
#[derive(Deserialize)]
pub struct Response {
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    pub txs: Vec<Transaction>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Transaction {
    pub logs: Vec<Log>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Log {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Event {
    pub attributes: Vec<Attribute>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

// // Models for saving state data
// #[derive(Deserialize)]
// pub struct Borrower {
//     address: String,
//     total_borrowed: u128,
//     borrow_events: Vec<Event>
// }
