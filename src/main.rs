mod cli;

use clap::Parser;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

use crate::cli::Command;

const DIE_SIZE: u32 = 6;
const INDEX_LENGTH: u32 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    match args.command {
        Command::Phrase { length, separator } => {
            let dictionary = load_dictionary();

            for _ in 0..args.count {
                let mut words = vec![];

                for _ in 0..length {
                    let word = get_word(&dictionary);
                    words.push(word);
                }

                println!("{}", words.join(&separator));
            }
            Ok(())
        }
        Command::Token { length: _ } => Ok(()),
    }
}

fn load_dictionary() -> HashMap<String, String> {
    let file = File::open("./eff_large_wordlist.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut dictionary = HashMap::new();

    for result in lines {
        let line = result.unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        dictionary.insert(String::from(parts[0]), String::from(parts[1]));
    }

    dictionary
}

fn get_word(dictionary: &HashMap<String, String>) -> &str {
    let index = get_index();
    dictionary.get(&index).unwrap()
}

fn get_index() -> String {
    let mut rolls = vec![];
    for _ in 0..INDEX_LENGTH {
        rolls.push(roll());
    }
    rolls.iter().map(|n| n.to_string()).collect()
}

fn roll() -> u32 {
    rand::random::<u32>() % DIE_SIZE + 1
}
