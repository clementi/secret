use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        long = "number",
        short = 'n',
        help = "Number of tokens or phrases to generate",
        default_value_t = 1
    )]
    pub count: u32,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Generate a token (string) of random characters")]
    Token {
        #[arg(long, short, help = "Length of token", default_value_t = 20)]
        length: u32,

        #[arg(long, short, help = "Alphabet to use", default_value_t = String::from("all"))]
        alphabet: String,
    },
    #[command(about = "Generate a passphrase of random words")]
    Phrase {
        #[arg(long, short, help = "Length of phrase (in words)", default_value_t = 4)]
        length: u32,

        #[arg(long, short, help = "Word separator", default_value_t = String::from(" "))]
        separator: String,
    },
}
