use std::collections::HashMap;
use std::hash::Hash;
use std::vec;

fn main() {
    println!("----------------------------------------------");
    println!("New empty vector");
    let vector: Vec<i32> = Vec::new();

    println!("----------------------------------------------");
    println!("New vector with inital values");
    let mut vector = vec![1, 2, 3];
    vector.push(4);

    println!("----------------------------------------------");
    println!("Reading Elements of Vectors");
    // using [index]
    let first = &vector[2];
    println!("First is: {}", first);

    // using .get(index)
    let third = vector.get(2);
    println!("Third is: {:?}", third);

    println!("----------------------------------------------");
    println!("Mutable/Immutable References at the same time not allowed");
    let mut v = vec![1, 2];
    // cannot have an immutable reference since .push() is a mutable reference
    // let first = &v[0];
    v.push(6);
    println!("first is: {first}");

    println!("----------------------------------------------");
    println!("Iterating Over the Vector & Dereferencing");
    for value in &mut v {
        *value += 2;
        println!("{value}");
    }

    println!("----------------------------------------------");
    println!("Using an Enum to Store Multiple Values");
    enum SpreadsheetCell {
        Int(i32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("----------------------------------------------");
    println!("Concat with + or format! Macro");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used, s2 is still valid
    println!("{}", s3);

    let s1 = String::from("Hello, ");
    let s = format!("{s1}{s2}");
    println!("{}", s);

    println!("----------------------------------------------");
    println!("Methods for Iterating Over Strings");
    let hello = "уйд";
    for char in hello.chars() {
        println!("{char}");
    }

    for byte in hello.bytes() {
        println!("{byte}");
    }

    println!("----------------------------------------------");
    println!("Creating a New Hash Map");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);

    println!(
        "Score of Blue team is: {:?}",
        scores.get(&String::from("Blue"))
    );

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of team {} is {}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("----------------------------------------------");
    println!("HashMaps and Ownership");
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value); // field_name and field_value are invalid
                                           // at this time
    println!("{field_name}: {field_value}");

    println!("----------------------------------------------");
    println!("Adding a Key and Value Only if a Key Isn't Present");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Black")).or_insert(9);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    println!("----------------------------------------------");
    println!("Updating a Value Based on the Old Value");
    let text = "hello world wonderful world";
    let mut word_to_count_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_to_count_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_to_count_map);
}
