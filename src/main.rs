mod alphabet;
mod cli;
mod dictionary;
mod phrase;

use clap::Parser;
use std::error::Error;

use crate::cli::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    match args.command {
        Command::Phrase { length, separator } => {
            let dictionary = dictionary::load_dictionary()?;

            for _ in 0..args.count {
                println!(
                    "{}",
                    phrase::generate_phrase(length, &separator, &dictionary)
                );
            }
            Ok(())
        }
        Command::Token { length, alphabet } => {
            let chars: Vec<char> = alphabet::get_alphabet(&alphabet).chars().collect();

            for _ in 0..args.count {
                let mut token_chars = vec![];

                for _ in 0..length {
                    let index = rand::random::<usize>() % chars.len();
                    let char = chars[index];
                    token_chars.push(char);
                }

                println!("{}", String::from_iter(token_chars));
            }
            Ok(())
        }
    }
}
