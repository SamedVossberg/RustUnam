use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); // Getting command line arguments

    // Ensure filename is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // getting the filename 
    let filename = &args[1];
    let file = File::open(filename)?;

    // variable to store word count
    let mut word_count = HashMap::new();

    // Declare reader
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?; // reading the lines
        for word in line.split_whitespace() { //white space sep
            let word = word.to_string();
            *word_count.entry(word).or_insert(0) += 1; // word counter

        }
    }
    // printing the results
    for word in word_count.keys() {
        println!("{}: {}", word, word_count[word]);
    }

    Ok(())

}


