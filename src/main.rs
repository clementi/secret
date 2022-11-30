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
            let file = File::open("./eff_large_wordlist.txt")?;
            let lines = io::BufReader::new(file).lines();

            let mut dictionary = HashMap::new();

            for result in lines {
                let line = result?;
                let parts: Vec<&str> = line.split('\t').collect();
                dictionary.insert(String::from(parts[0]), String::from(parts[1]));
            }

            for _ in 0..args.count {
                let mut words = vec![];

                for _ in 0..length {
                    let index = get_index();
                    let word = dictionary.get(&index).unwrap();

                    words.push(word.as_str());
                }

                println!("{}", words.join(&separator));
            }
            Ok(())
        }
        Command::Token { length: _ } => Ok(()),
    }
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
