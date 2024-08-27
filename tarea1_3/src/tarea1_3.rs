use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); // Getting command line arguments

    // Ensure filename is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename> [w|c]", args[0]);
        std::process::exit(1);
    }

    // getting the filename 
    let filename = &args[1];

    // integrating the option here
    let sort_option = if args.len() > 2 { &args[2] } else { "c" };

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

    let mut word_vec: Vec<(&String, &usize)> = word_count.iter().collect();
    match sort_option {
        "w" => word_vec.sort_by(|a, b| a.0.cmp(b.0)), // sorting by word here
        "c" => word_vec.sort_by(|a, b| b.1.cmp(a.1)), // sorting by frequency here
        _ => eprintln!("Opción no reconocida, se ordenará por frecuencia"),
    }

    // printing the results
    for (word, &count) in word_vec {
        println!("{} {}", word, count);
    }

    Ok(())

}
