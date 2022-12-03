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
            let dictionary = dictionary::load_dictionary();

            for _ in 0..args.count {
                println!(
                    "{}",
                    phrase::generate_phrase(length, &separator, &dictionary)
                );
            }
            Ok(())
        }
        Command::Token {
            length: _,
            alphabet: _,
        } => Ok(()),
    }
}
