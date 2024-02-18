fn main() {
    println!("----------------------------------------------------");
    println!("Variables");
    let mut x = 5;
    println!("x: {x}");

    x = 6;
    println!("x: {x}");

    println!("----------------------------------------------------");
    println!("Constants");
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    println!("----------------------------------------------------");
    println!("Shadowing");
    // 1
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("x in inner scope: {x}");
    }

    println!("x in outer scope: {x}");

    // 2
    let num = 5;

    {
        let num = "five";
        println!("num: {num}");
    }

    println!("num: {num}");

    println!("----------------------------------------------------");
    println!("Tuples");
    let tup = ("Taha", 24, "Sioux Falls");

    let (name, age, location) = tup;
    println!("{name}, {age}, {location}.");

    let name = tup.0;
    let age = tup.1;
    let location = tup.2;
    println!("{name}, {age}, {location}.");

    println!("----------------------------------------------------");
    println!("Arrays");
    let array = [1.0, 2.3, 4.5];
    let index1 = array[1];
    println!("{index1}"); // can't do this: println!("{array[1]}");

    let array = [5; 10];
    let index9 = array[9];
    println!("{index9}");

    println!("----------------------------------------------------");
    println!("Functions");

    print_name_and_age("Taha", 24);
    print_name_and_age_2(String::from("Taha"), 24);

    println!("----------------------------------------------------");
    println!("Return values");
    println!("{}", get_five());

    println!("----------------------------------------------------");
    println!("if");

    // 1
    let number = 9;
    if number % 2 == 0 {
        println!("divisible by 2");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 9 == 0 {
        println!("divisible by 9");
    } else {
        println!("not divisible by any");
    }

    // 2
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}");

    println!("----------------------------------------------------");
    println!("Loops");

    // 1
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 3 {
            break counter * 2;
        }
    };
    println!("result: {result}");

    // 2
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("count: {count}");

    // 3
    let mut num = 3;
    while num != 0 {
        println!("{}", num);
        num -= 1;
    }
    println!("Lift off!");

    // 4
    let a = [1, 2, 3];
    for element in a {
        println!("{}", element);
    }

    for number in (1..=3).rev() {
        println!("{}", number);
    }
    println!("Lift off!");
}

// (parameter: type) as opposed to (type paramater) in some languages
fn print_name_and_age(name: &str, age: i32) {
    println!("{name}: {age}");
}

fn print_name_and_age_2(name: String, age: i32) {
    println!("{name}: {age}");
}

// Or I can do return 5; in the body
// I cannot do 5; because that is a statement and statements do no evaluate to a value
fn get_five() -> i32 {
    5
}
