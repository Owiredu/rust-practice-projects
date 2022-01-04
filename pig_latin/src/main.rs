mod lib;
use lib::io_funcs;
use std::{io, io::Write};

fn main() {
    // print the program banner
    println!(
        "\n{:*^51}\n",
        " Welcome to the Pig Latin Text Generator "
    );

    // the main program loop
    'program: loop {
        // show input prompt
        print!("Input string: ");
        io::stdout().flush().expect("Failed to flush output");
    
        // get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
    
        // validate the string before converting it
        if input.is_empty() {
            // handle empty string
            println!("Error: Empty string");
        } else {
            // get pig latin if possible
            input = input.trim().to_string();
            match to_pig_latin(&input) {
                Ok(pig_latin_output) => println!("Result: {}", pig_latin_output),
                _ => {},
            }
        }
    
        // make exit decision
        'exit: loop {
            match io_funcs::get_exit_option() {
                Ok(opt) => {
                    if opt == 1 {
                        // rerun program
                        continue 'program;
                    } else if opt == 2 {
                        // rerun program
                        break 'program;
                    } else {
                        // rerun exit block
                        continue 'exit;
                    }
                }
                Err(_) => {
                    // rerun exit block
                    continue 'exit;
                }
            }
        }
    }
}

/// Convert the input string to pig latin
fn to_pig_latin(input: &String) -> Result<String, ()> {
    // create the pig latin string
    let mut pig_latin_output = String::new();
    // loop through the words in the input
    for word in input.split_terminator(" ") {
        // skip if the word is empty
        if word.to_string().trim() == "" { continue; }

        // check if the first character is a consonant or vowel
        match word.to_ascii_lowercase().chars().next() {
            Some('a' | 'e' | 'i' | 'o' | 'u') => {
                // append '-hay' to the end of the word
                pig_latin_output.push_str(&format!("{}-hay ", word));
            },
            Some('a'..='z') => {
                // separate the first character from the remaining string
                let (first, last) = word.split_at(1);
                // compose the pig latin string
                pig_latin_output.push_str(&format!("{}-{}ay ", last, first));
            },
            _ => {
                // print the error message
                println!("Error: Invalid alphabet is starting word \"{}\".", word);
                // return error result
                return Err(());
            }
        }
    }
    // return Ok result containing the pig latin text
    Ok(pig_latin_output)
}