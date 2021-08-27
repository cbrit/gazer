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

// Unsure how to figure out the ErrorKind that from_str returns, so 
// we'll just log failed deserializations and move on.
fn get_transactions(json: String) -> Option<Vec<Transaction>> {
    let resp: Response = match serde_json::from_str(json.as_str()) {
        Ok(r) => r,
        Err(err) =>  {
            eprintln!("{}", err);
            return None
        },
    };

    Some(resp.data.txs)
}

fn get_borrow_events(txs: Vec<Transaction>) -> Option<Vec<Event>> {
    
}

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
        "#.to_string();

        let mut expected = Vec::new();
        expected.push(Transaction { logs: vec![] });
        expected.push(Transaction { logs: vec![] });
        let actual = get_transactions(j);

        assert_eq!(expected, actual.unwrap());
    }
    
    #[test]
    fn get_transactions_returns_none_if_deserialization_fails() {
        let expected = None;
        let actual = get_transactions("surprise!".to_string());

        assert_eq!(expected, actual);
    }
}
