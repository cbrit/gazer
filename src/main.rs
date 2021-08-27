use std::process;

fn main() {
    if let Err(e) = gazer::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}