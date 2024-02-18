use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius; or the other way around :)");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let choice: i32 = get_menu_selection();

    if choice == 1 {
        println!("Enter the temperature in Celsius:");
        let user_input = get_user_input();
        let result = celsius_to_fahrenheit(user_input);
        println!("Result: {:.2} Fahrenheit", result);
    } else if choice == 2 {
        println!("Enter the temperature in Fahrenheit:");
        let user_input = get_user_input();
        let result = fahrenheit_to_celsius(user_input);
        println!("Result: {:.2} Celsius", result);
    } else {
        println!("Invalid choice. Please select 1 or 2.");
    }
}

fn get_menu_selection() -> i32 {
    loop {
        let mut menu_selection = String::new();

        io::stdin()
            .read_line(&mut menu_selection)
            .expect("Failed to read line");

        match menu_selection.trim().parse() {
            Ok(num) => {
                if num == 1 || num == 2 {
                    return num;
                }
                println!("Please enter 1 or 2");
            }
            Err(_) => {
                println!("Please enter a valid number");
            }
        }
    }
}

fn get_user_input() -> f64 {
    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
