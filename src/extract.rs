use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::models::*;

// Unsure how to figure out the ErrorKind that from_str returns, so 
// we'll just log failed deserializations and move on.
pub fn get_transactions(json: String) -> Option<Vec<Transaction>> {
    let resp: Response = match serde_json::from_str(json.as_str()) {
        Ok(r) => r,
        Err(err) =>  {
            eprintln!("{}", err);
            return None
        },
    };
    
    Some(resp.data.txs)
}

// Look at using Iterators and adaptors to refactor this.
pub fn get_borrow_events(txs: &Vec<Transaction>) -> Option<Vec<Event>> {
    let mut results: Vec<Event> = Vec::new();

    for tx in txs {
        for log in &tx.logs {
            'event_loop: for event in log.events.clone() {
                for attr in &event.attributes {
                    if attr.value == "borrow_stable".to_string() {
                        results.push(event);
                        continue 'event_loop;
                    }
                }
            }
        }
    }

    if results.len() > 0 {
        return Some(results);
    }
    
    None
}

pub fn handle_extract_borrow_data(obs_rx: Arc<Mutex<Receiver<String>>>, tx: Sender<Vec<Borrower>>) {
    loop {
        let receiver = obs_rx.clone();
        let sender = tx.clone();
        let data_thread = thread::spawn(move || {
            loop {
                let json = receiver.lock().unwrap().recv().unwrap().trim().to_string();
                let txs = match get_transactions(json) {
                    Some(result) => result,
                    None => {
                        eprintln!("transactions yielded None");
                        continue;
                    },
                };

                let borrow_events = match get_borrow_events(&txs) {
                    Some(events) => events,
                    None =>  {
                        println!("No borrow events");
                        continue;
                    },
                };

                let borrowers: Vec<Borrower> = borrow_events.into_iter().map(Borrower::new).collect();

                println!("Borrowers: {:?}", borrowers);
                sender.send(borrowers);
            }
        });

        data_thread.join().unwrap();
    }
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

    #[test]
    fn get_borrow_events_returns_some_if_present() {
        // I'm sure there is a better way to arrange this.
        let expected_event1 = Event { attributes: vec!(Attribute { key: "action".to_string(), value: "borrow_stable".to_string()})};
        let expected_event2 = Event { attributes: vec!(Attribute { key: "action".to_string(), value: "borrow_stable".to_string()})};
        let expected = vec!(expected_event1, expected_event2);
        let actual_attrs1 = vec!(Attribute { key: "action".to_string(), value: "borrow_stable".to_string()});
        let actual_attrs2 = vec!(Attribute { key: "action".to_string(), value: "borrow_stable".to_string()});
        let txs = vec!(Transaction { logs: vec!(Log { events: vec!(Event { attributes: actual_attrs1}, Event { attributes: actual_attrs2})})});
        let actual = get_borrow_events(&txs).unwrap();

        assert_eq!(expected.len(), actual.len());
        assert_eq!(expected[1], actual[1]);
    }

    #[test]
    fn get_borrow_events_returns_none() {
        let actual_attr = vec!(Attribute { key: "action".to_string(), value: "not_borrow".to_string()});
        let txs = vec!(Transaction { logs: vec!(Log { events: vec!(Event { attributes: actual_attr})})});
        let actual = get_borrow_events(&txs);

        assert_eq!(None, actual);
    }
}
