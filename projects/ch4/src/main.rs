fn main() {
    println!("-----------------------------------------------");
    println!("Ownership");

    let s1 = String::from("Hello");
    // without clone(), s1 will be moved into s2 making s1 invalid.
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    // clone() isn't needed because we know the size of i1 & i2 at compile time and they
    // are stored on the stack.
    // i1 is still valid.
    let i1 = 5;
    let i2 = i1;

    println!("{}", i1);
    println!("{}", i2);

    let s = String::from("Hello");
    takes_ownership(s); // s is moved to takes_ownership and can't be used anymore.

    let _: String = gives_ownership();

    let s2 = String::from("another string");
    let _ = takes_and_gives_back(s2);

    println!("-----------------------------------------------");
    println!("References");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The size of \"{s1}\" is {len}");

    println!("-----------------------------------------------");
    println!("Mutable References");
    let mut s1 = String::from("Hello");
    append_to_string(&mut s1);

    // restriction in mutable references
    let mut s = String::from("Hello");
    let _r1 = &mut s;
    // let r2 = &mut s; // cannot do this

    // allowing multiple references
    let mut s = String::from("Hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here

    let _r2 = &mut s;

    println!("-----------------------------------------------");
    println!("Dangling References");
    // Can't return a reference from dangle(). Return s, so
    // that ownership is transferred and all is happy
    let _reference_to_nothing = dangle();

    println!("-----------------------------------------------");
    println!("The Rules of References");
    println!("String Slices");
    let mut s = String::from("Hello world");
    let _index = first_space_index(&s); // index will have value 5

    s.clear(); // this empties `s` making it ""
               // but index still has value 5, but there's no more string that we could use the
               // value 5 with. index is no longer valid!
               // If we use index to extract the word from s, it will cause a bug.

    // string_slice implements the correct solution & makes the signature more general.
    let my_string = String::from("Hello world");
    // it works on slices of `String`s, whether partial or whole
    let _word = string_slice(&my_string[0..6]);
    let _word = string_slice(&&my_string[..]);
    let _word = string_slice(&my_string);

    // it also works on string literals, whether partial or whole
    let my_string = "hello world";
    let _word = string_slice(&my_string[0..6]);
    let _word = string_slice(&my_string[..]);
    // and b/c string literals are string literals, this works too
    let _word = string_slice(my_string);

    println!("Other Slices");
    
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn gives_ownership() -> String {
    String::from("Taha")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

fn append_to_string(string: &mut String) {
    string.push_str(", world!");
    println!("{string}");
}

fn dangle() -> String {
    let s = String::from("dangle");

    s
}

// Question: Write a function that takes a string of words seprated by spaces and returns
// the index of the first space. If no space is found, return the whole string must be considered as
// one word, so the last character's index should be returned.
fn first_space_index(string: &String) -> usize {
    let bytes = string.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }

    string.len()
}

fn string_slice(string: &str) -> &str {
    let bytes = string.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &string[0..index];
        }
    }

    &string[..]
}
