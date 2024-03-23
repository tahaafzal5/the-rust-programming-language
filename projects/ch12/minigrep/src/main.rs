use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = minigrep::run(config) {
        println!("Application error: {error}");
        process::exit(1);
    };
}
