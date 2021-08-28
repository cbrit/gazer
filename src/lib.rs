pub mod models;
pub mod extract;

use models::{Borrower};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("response.json").unwrap();
    let txs = extract::get_transactions(json).unwrap();
    let borrow_events = extract::get_borrow_events(&txs).unwrap();
    let borrowers: Vec<Borrower> = borrow_events.into_iter().map(Borrower::new).collect();

    println!("{:?}", borrowers);

    Ok(())
}

