// Models for drilling down into the JSON response
#[derive(Deserialize)] struct Response { data: Data }
#[derive(Deserialize)] struct Transaction { logs: Vec<Log> }
#[derive(Deserialize)] struct Data { txs: Vec<Transaction> }
#[derive(Deserialize)] struct Log { events: Vec<Event> }
#[derive(Deserialize)] struct Event { attributes: Vec<Attribute> }
#[derive(Deserialize)] struct Attribute { key: String, value: String, }

// Models for saving state data
#[derive(Deserialize)] 
struct Borrower {
    address: String,
    total_borrowed: u128,
    borrow_events: Vec<Event>
}