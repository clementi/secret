mod cli;

use std::error::Error;

use clap::Parser;

const DIE_SIZE: u32 = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();

    println!("{:?}", args);

    for _ in 0..4 {
        let mut rolls = vec![];
        for _ in 0..5 {
            rolls.push(rand::random::<u32>() % DIE_SIZE + 1);
        }
        let index = rolls.iter().map(|n| n.to_string()).collect::<String>();
        println!("{}", index);
    }

    Ok(())
}
