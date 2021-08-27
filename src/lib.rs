pub mod models;

use models::{Response, Transaction, Log, Event, Attribute};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("response.json").unwrap();
    let txs = get_transactions(json);
    // let borrow_events = get_borrow_events(txs);

    Ok(())
}

fn get_transactions(json: String) -> Vec<Transaction> {
    let resp: Response = serde_json::from_str(json.as_str()).expect("failed to deserialize json to Response");
    let txs: Vec<Transaction> = resp.data.txs;
    txs
}

// fn get_borrow_events(txs: Vec<Transaction>) -> Vec<Event> {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

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
