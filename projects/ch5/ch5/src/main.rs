use std::num::ParseIntError;

fn main() {
    println!("------------------------------------------------");
    println!("Structs");
    let mut user1 = User {
        active: true,
        username: String::from("po.nka"),
        email: String::from("po@test.com"),
        sign_in_count: 0,
    };

    user1.sign_in_count = 1;

    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);

    println!("Struct Update Syntax");
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("new_email@test.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("new_email@test.com"),
        ..user1 // can no longer use user1; see notes
    };

    println!("------------------------------------------------");
    println!("Tuple Structs");

    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    println!("{}", black.1);
    println!("{}", point.2);

    println!("------------------------------------------------");
    println!("Unit-Like Structs without Any Fields");
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 0,
    }
}

// Field Init Shorthand
fn _build_user_2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// In the future, we'll implement behavior for this type such that
// every instance of `AlwaysEqual` is always to another instance of any other
// type. We wouldn't need any data to implement that behavior.
struct AlwaysEqual;
