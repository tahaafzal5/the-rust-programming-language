use std::{error::Error, fs};

use crate::config::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = search(config.query, &file_contents);

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

fn search<'a>(query: String, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file_contents.lines() {
        if line.contains(&query) {
            results.push(line.trim());
        }
    }

    results
}
