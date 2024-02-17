use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess:");

    let mut tries = 1;
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Ignoring a non-number input and asking for another guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Too small!");
                tries = tries + 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                tries = tries + 1;
            }
            Ordering::Equal => {
                println!("You win! It took you {tries} tries.");
                // End the game when the user guesses the correct number
                break;
            }
        }
    }
}
