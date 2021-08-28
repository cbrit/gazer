use serde_derive::Deserialize;

pub const BORROW_ACTION: &str = "borrow_stable";
pub const BORROW_AMOUNT_KEY: &str = "borrow_amount";
pub const BORROWER_KEY: &str = "borrower";
pub const CONTRACT_ADDR_KEY: &str = "contract_address";

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

#[derive(Clone, Deserialize, Debug, PartialEq)]
// make Event type enum with BorrowEvent variant?
pub struct Event {
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

// Models for saving state data
#[derive(Debug, Deserialize)]
pub struct Borrower {
    pub address: String,
    pub total_borrowed: u128,
    pub borrow_events: Vec<Event>
}

impl Borrower {
    pub fn new(event: Event)  -> Self {
        if Self::is_borrow_event(&event) == false {
            // Need better error handling here
            panic!("attempted to construct a Borrower from invalid Event");
        }

        let addr = event.attributes.iter()
            .find(|a| a.key == BORROWER_KEY)
            .unwrap()
            .value
            .as_str()  // this is gross but I needed a quick fix for
            .to_string();   // "can't move out of shared reference" error
        let amount: u128 = event.attributes.iter()
            .find(|a| a.key == BORROW_AMOUNT_KEY)
            .unwrap()
            .value
            .parse()
            .unwrap();
        let mut events = Vec::new();
        events.push(event);

        Self {
            address: addr,
            total_borrowed: amount,
            borrow_events: events,
        }
    }

    pub fn is_borrow_event(event: &Event) -> bool {
        
        event.attributes
            .iter()
            .any(|a| 
                a.value == BORROW_ACTION.to_string()
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrower_new_constructs() {
        let expected = Borrower {
            address: "me".to_string(),
            total_borrowed: 1337,
            borrow_events: vec!(Event { 
                attributes: vec!(
                    Attribute { key: "action".to_string(), value: "borrow_stable".to_string() },
                    Attribute { key: "borrower".to_string(), value: "me".to_string() },
                    Attribute { key: "borrow_amount".to_string(), value: "1337".to_string() },
                    Attribute { key: "contract_address".to_string(), value: "uranus".to_string() }
                )
            }),
        };
        let event = Event { 
            attributes: vec!(
                Attribute { key: "action".to_string(), value: "borrow_stable".to_string() },
                Attribute { key: "borrower".to_string(), value: "me".to_string() },
                Attribute { key: "borrow_amount".to_string(), value: "1337".to_string() },
                Attribute { key: "contract_address".to_string(), value: "uranus".to_string() }
            )
        };

        let actual = Borrower::new(event);
        
    }

    #[test]
    fn is_borrow_event_returns_true() {
        let event = Event {
            attributes: vec!(Attribute {
                key: "action".to_string(),
                value: BORROW_ACTION.to_string(),
            })
        };
        let actual = Borrower::is_borrow_event(&event);

        assert_eq!(true, actual);
    }

   #[test]
    fn is_borrow_event_returns_false() {
        let event = Event {
            attributes: vec!(Attribute {
                key: "action".to_string(),
                value: "something_else".to_string(),
            })
        };
        let actual = Borrower::is_borrow_event(&event);

        assert_eq!(false, actual);
    }
}
