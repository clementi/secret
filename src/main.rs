mod alphabet;
mod cli;
mod word_list;
mod phrase;
mod token;

use clap::Parser;
use std::error::Error;

use crate::cli::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    match args.command {
        Command::Phrase { length, separator } => {
            let word_list = word_list::load_word_list()?;

            for _ in 0..args.count {
                println!(
                    "{}",
                    phrase::generate_phrase(length, &separator, &word_list)
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
