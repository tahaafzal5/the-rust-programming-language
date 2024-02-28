// Exercise: Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!
use std::io;

fn starts_with_vowels(word: &String) -> bool {
    let vowels = ["a", "e", "i", "o", "u"];

    vowels.iter().any(|vowel| word.starts_with(vowel))
}

fn main() {
    let mut input = String::new();

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let tokenized_input: Vec<&str> = input.trim().split_whitespace().collect();

        let mut output: Vec<String> = Vec::new();
        for token in tokenized_input {
            let mut token = token.to_string();

            if starts_with_vowels(&token) {
                token.push_str("-hay");
            } else {
                let first_char = token.remove(0);
                token = format!("{}-{}ay", token, first_char);
            }

            output.push(token.clone());
        }

        println!("{:?}", output);
    }
}
