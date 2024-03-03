use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("----------------------------------------");
    println!("Panic in our code");
    // panic!("crash and burn");

    println!("----------------------------------------");
    println!("Panic cause we use a library wrong");
    let vec = vec![1, 2, 3];
    // vec[99];

    println!("----------------------------------------");
    println!("Recoverable Errors with Result");
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    println!("----------------------------------------");
    println!("Matching on Different Errors");
    let greeting_file_result = File::open("./src/hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => {
    //         println!("File opened");
    //         file
    //     }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("./src/hello.txt") {
    //             Ok(file) => {
    //                 println!("File created");
    //                 file
    //             }
    //             Err(error) => panic!("Problem creating the file: {:?}", error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    println!("----------------------------------------");
    println!("Alternatives to Using match with Result<T, E>");
    // let greeting_file = File::open("./src/hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         println!("File not found");
    //         let file = File::create("./src/hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         });
    //         println!("File created");
    //         file
    //     } else {
    //         panic!("Problem opening file: {:?}", error);
    //     }
    // });

    println!("----------------------------------------");
    println!("Shortcuts for Panic on Error: unwrap and expect");

    // let greeting_file = File::open("./src/hello.txt").unwrap();
    // let greeting_file = File::open("./src/hello.txt").expect("Can't open file......");

    println!("----------------------------------------");
    println!("Propagating Errors -- see read_username_from_file()");
    // let username = match read_username_from_file() {
    //     Ok(username) => username,
    //     Err(_) => panic!("No username found -- crashing"),
    // };
    // println!("Found username from file");

    println!("----------------------------------------");
    println!(
        "A Shortcut for Propagating Errors: The ? Operator -- see read_username_from_file_short()"
    );
    // let username = match read_username_from_file_short() {
    //     Ok(username) => username,
    //     Err(_) => panic!("No username found in short func -- crashing"),
    // };
    // println!("Found username from file");

    println!("----------------------------------------");
    println!("Where the ? Operator Can Be Used -- see last_char_of_first_line()");

    println!("----------------------------------------");
    println!("Creating Custom Types for Validation");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("./src/username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    // the ? will return the value inside an Ok to the variable username_file.
    // If an error occurs, it will return early out of the whole function and
    // give any Err value to the calling code.
    let mut username_file = File::open("./src/username.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_short_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("./src/username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("./src/username.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// fn main() -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    // this is a getter: it gets some data from its fields and returns it.
    // This public method is necessary because the value field of the
    // Guess struct is private.
    // It’s important that the value field be private so code using the
    // Guess struct is not allowed to set value directly: code outside the
    // module must use the Guess::new function to create an instance of Guess.
    pub fn value(&self) -> i32 {
        self.value
    }
}
