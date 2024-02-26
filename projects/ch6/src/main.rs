fn main() {
    println!("-----------------------------------------------");
    println!("Enums");
    let _home = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("127.0.0.1"),
    };

    // putting data directly into each enum variant.
    let _home = IpAddrUsingEnum::V4(127, 0, 0, 1);
    let _loopback = IpAddrUsingEnum::V6(String::from("::1"));

    // enums with different types of data
    // defining methods on enums
    let hello_message = Message::Write(String::from("hello"));
    let quit_message = Message::Quit;
    hello_message.call();
    quit_message.call();

    println!("-----------------------------------------------");
    println!("Option Enum");
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("{:?}", some_number);
    println!("{:?}", absent_number);

    println!("-----------------------------------------------");
    println!("Match");
    let dime = Coin::Dime;
    println!("{}", value_in_cents(dime));

    println!("-----------------------------------------------");
    println!("Patterns that bind to values");
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(alaska_quarter));

    println!("-----------------------------------------------");
    println!("Matching with `Option<T>`");
    let five = Some(5);
    println!("{:?}", plus_one(five));
    println!("{:?}", plus_one(None));

    println!("-----------------------------------------------");
    println!("Catch-All Patterns and the _ Placeholder");
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => take_default_action(other),
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    println!("-----------------------------------------------");
    println!("Concise Control Flow with if let");
    let config_max = Some(3);
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("The max is configured to be {max}");
    }

    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        println!("Not a quarter... :(");
    }
}

enum IpAddrKind {
    _V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrUsingEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // no data associated with it
    Move { x: i32, y: i32 },    // 2 named fields
    Write(String),              // single String
    ChangeColor(i32, i32, i32), // 3 i32 values
}

impl Message {
    fn call(&self) {
        println!("called!");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn take_default_action(num: u8) {
    println!("Take default action on num {num}");
}
