use log::{debug, error, info};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::models::*;

// Unsure how to figure out the ErrorKind that from_str returns, so
// we'll just log failed deserializations and move on.
pub fn get_transactions(json: String) -> Option<Vec<Transaction>> {
    let resp: Response = match serde_json::from_str(&json.as_str()) {
        Ok(r) => r,
        Err(err) => {
            error!("{}", err);
            return None;
        }
    };

    let resp: Vec<Transaction> = resp
        .data
        .txs
        .into_iter()
        .filter(|t| t.logs.is_some())
        .collect();

    Some(resp)
}

pub fn get_borrow_events(txs: Vec<Transaction>) -> Option<Vec<Event>> {
    let results: Vec<Event> = txs
        .into_iter()
        .flat_map(|tx| tx.logs)
        .into_iter()
        .flatten()
        .flat_map(|log| log.events)
        .filter(|event| {
            event
                .attributes
                .iter()
                .any(|attr| attr.value == "borrow_stable")
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    Some(results)
}

pub fn handle_extract_borrow_data(obs_rx: Arc<Mutex<Receiver<String>>>, tx: Sender<Vec<Borrower>>) {
    loop {
        let receiver = obs_rx.clone();
        let sender = tx.clone();
        let data_thread = thread::spawn(move || loop {
            let json = receiver.lock().unwrap().recv().unwrap().trim().to_string();
            let txs = match get_transactions(json) {
                Some(result) => result,
                None => {
                    debug!("transactions yielded None");
                    continue;
                }
            };

            let borrow_events = match get_borrow_events(txs) {
                Some(events) => events,
                None => {
                    info!("Block contained no borrow events");
                    continue;
                }
            };

            let borrowers: Vec<Borrower> = borrow_events.into_iter().map(Borrower::new).collect();

            info!("Borrowers: {:?}", borrowers);
            sender.send(borrowers).unwrap_or_else(|err| {
                error!("{:?}", err);
            });
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
        "#
        .to_string();

        let mut expected = Vec::new();
        expected.push(Transaction { logs: Some(vec![]) });
        expected.push(Transaction { logs: Some(vec![]) });
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
        let expected_event1 = Event {
            attributes: vec![Attribute {
                key: "action".to_string(),
                value: "borrow_stable".to_string(),
            }],
        };
        let expected_event2 = Event {
            attributes: vec![Attribute {
                key: "action".to_string(),
                value: "borrow_stable".to_string(),
            }],
        };
        let expected = vec![expected_event1, expected_event2];
        let actual_attrs1 = vec![Attribute {
            key: "action".to_string(),
            value: "borrow_stable".to_string(),
        }];
        let actual_attrs2 = vec![Attribute {
            key: "action".to_string(),
            value: "borrow_stable".to_string(),
        }];
        let txs = vec![Transaction {
            logs: Some(vec![Log {
                events: vec![
                    Event {
                        attributes: actual_attrs1,
                    },
                    Event {
                        attributes: actual_attrs2,
                    },
                ],
            }]),
        }];
        let actual = get_borrow_events(txs).unwrap();

        assert_eq!(expected.len(), actual.len());
        assert_eq!(expected[1], actual[1]);
    }

    #[test]
    fn get_borrow_events_returns_none() {
        let actual_attr = vec![Attribute {
            key: "action".to_string(),
            value: "not_borrow".to_string(),
        }];
        let txs = vec![Transaction {
            logs: Some(vec![Log {
                events: vec![Event {
                    attributes: actual_attr,
                }],
            }]),
        }];
        let actual = get_borrow_events(txs);

        assert_eq!(None, actual);
    }
}
