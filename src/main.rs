pub mod models;

use models::{Response, Transaction, Log, Event, Attribute};
use std::fs;

fn main() {
    let json = read_file("response.json");
    let txs = get_transactions(json);
    // let borrow_events = get_borrow_events(txs);
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

fn get_transactions(json: &str) -> Vec<Transaction> {
    let resp: Response = serde_json::from_str(json).expect("failed to deserialize json to Response");
    let txs: Vec<Transaction> = resp.data.txs;
    txs
}

// fn get_borrow_events(txs: Vec<Transaction>) -> Vec<Event> {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_file_content() {
        let expected = fs::read_to_string("response.json").unwrap();
        let actual = read_file("response.json");

        assert_eq!(expected, actual);
    }

    #[test]
    fn get_transactions_returns_transactions() {
        let j = r#"
{
    "data": {
        "txs": [
            {
                "logs": []
            },
            {
                "logs": []
            }
        ]
    }
}
        "#;

        let mut expected = Vec::new();
        expected.push(Transaction { logs: vec![] });
        expected.push(Transaction { logs: vec![] });
        let actual = get_transactions(j);

        assert_eq!(expected, actual);
    }
}
