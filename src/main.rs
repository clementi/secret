mod cli;
mod phrase;

use clap::Parser;
use std::error::Error;

use crate::cli::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    match args.command {
        Command::Phrase { length, separator } => {
            let dictionary = phrase::load_dictionary();

            for _ in 0..args.count {
                let mut words = vec![];

                for _ in 0..length {
                    let word = phrase::get_word(&dictionary);
                    words.push(word);
                }

                println!("{}", words.join(&separator));
            }
            Ok(())
        }
        Command::Token { length: _ } => Ok(()),
    }
}
