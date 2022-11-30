use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(
        long,
        short,
        help = "Number of tokens or phrases to generate",
        default_value_t = 1
    )]
    pub count: u32,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    Token { name: Option<String> },
    Phrase { name: Option<String> },
}
