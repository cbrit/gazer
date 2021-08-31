use std::process;

use gazer::Config;

const CONFIG_FILE: &str = "config.json";

fn main() {
    let config = Config::new(CONFIG_FILE).unwrap_or_else(|err| {
        eprintln!("Error constructing Config: {}", err);
        process::exit(1);
    });

    if let Err(e) = gazer::run(config) {
        eprintln!("Critical error: {}", e);
        process::exit(1);
    }
}