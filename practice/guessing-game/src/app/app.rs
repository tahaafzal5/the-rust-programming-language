use std::{cmp::Ordering, error::Error, io};

use crate::app::guess::Guess;
use rand::Rng;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Input a Guess between 0 and 100, inclusive");

    let mut user_input = String::new();

    let rng = rand::thread_rng().gen_range(0..=100);

    loop {
        user_input.clear();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read from stdin");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(value) => {
                if value < 0 || value > 100 {
                    println!("Input must be between 0 and 100, try again:");
                    continue;
                } else {
                    value
                }
            }
            Err(_) => continue,
        };

        let user_guess = Guess::new(user_input);

        match user_guess.value().cmp(&rng) {
            Ordering::Less => {
                println!("Your guess is too low. Try again!");
            }
            Ordering::Greater => {
                println!("Your guess is too high. Try again!");
            }
            Ordering::Equal => {
                println!("You guessed correctly!");
                return Ok(());
            }
        }
    }
}
