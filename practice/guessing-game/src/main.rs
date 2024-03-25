use std::process;

use guessing_game::app::app;

fn main() {
    println!("Welcome to Guessing Game");

    if let Err(error) = app::run() {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}
