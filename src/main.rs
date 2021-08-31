use std::process;
use log::{debug, error, log_enabled, info, Level};

use gazer::Config;

const CONFIG_FILE: &str = "config.json";

fn main() {
    env_logger::init();

    info!("Starting up");
    info!("Loading config"); 
    let config = Config::new(CONFIG_FILE).unwrap_or_else(|err| {
        error!("Error constructing Config: {}", err);
        process::exit(1);
    });

    info!("Begin run");
    if let Err(e) = gazer::run(config) {
        error!("Critical error: {}", e);
        process::exit(1);
    }
}