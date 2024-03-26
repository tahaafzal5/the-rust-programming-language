use std::{env, process};

use minigrep::app::app;
use minigrep::config::config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = app::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
