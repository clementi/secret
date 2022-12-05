mod alphabet;
mod cli;
mod dictionary;
mod phrase;
mod token;

use alphabet::InvalidOptionsError;
use clap::Parser;
use std::error::Error;

use crate::cli::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    if args.count < 1 {
        return Err(Box::new(InvalidOptionsError::new(
            "count must be at least 1",
        )));
    }

    match args.command {
        Command::Phrase { length, separator } => {
            if length < 1 {
                return Err(Box::new(InvalidOptionsError::new(
                    "length must be at least 1",
                )));
            }

            let dictionary = dictionary::load_dictionary()?;

            for _ in 0..args.count {
                println!(
                    "{}",
                    phrase::generate_phrase(length, &separator, &dictionary)
                );
            }
            Ok(())
        }
        Command::Token {
            length,
            alpha_lower,
            alpha_upper,
            alpha,
            numeric,
            alphanumeric,
            symbols,
            all,
        } => {
            if length < 1 {
                return Err(Box::new(InvalidOptionsError::new(
                    "length must be at least 1",
                )));
            }

            let mut options = alphabet::AlphabetOptions {
                alpha_lower,
                alpha_upper,
                alpha,
                numeric,
                alphanumeric,
                symbols,
                all,
            };

            if options.all_false() {
                options.all = true;
            }

            let chars: Vec<char> = alphabet::get_alphabet(&options).chars().collect();

            for _ in 0..args.count {
                println!("{}", token::generate_token(length, &chars));
            }
            Ok(())
        }
    }
}
