mod alphabet;
mod cli;
mod dictionary;
mod phrase;
mod token;

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
                println!("{}", token::generate_token(length, &chars));
            }
            Ok(())
        }
    }
}
