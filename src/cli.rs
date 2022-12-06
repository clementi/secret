use clap::{Parser, Subcommand};

fn greater_than_zero(v: &str) -> Result<u32, String> {
    let num: u32 = v.parse::<u32>().map_err(|_| String::from("--number must be an integer greater than zero"))?;
    if num > 0 {
        Ok(num)
    } else {
        Err(String::from("--number must be an integer greater than zero"))
    }
}

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
        default_value_t = 1,
        value_parser = greater_than_zero
    )]
    pub count: u32,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Generate a token (string) of random characters")]
    Token {
        #[arg(long, short, help = "Length of token", default_value_t = 20, value_parser = greater_than_zero)]
        length: u32,

        #[arg(long, help = "Use lowercase letters")]
        alpha_lower: bool,

        #[arg(long, help = "Use uppercase letters")]
        alpha_upper: bool,

        #[arg(
            long,
            help = "Use lowercase and uppercase letters (equivalent to --alpha-lower --alpha-upper)"
        )]
        alpha: bool,

        #[arg(long, help = "Use numeric characters")]
        numeric: bool,

        #[arg(
            long,
            help = "Use alphanumeric characters (equivalent to --alpha --numeric)"
        )]
        alphanumeric: bool,

        #[arg(long, help = "Use symbols")]
        symbols: bool,

        #[arg(
            short,
            long,
            help = "Use everything (equivalent to --alphanumeric --symbols) [default if no character sets are provided]"
        )]
        all: bool,
    },
    #[command(about = "Generate a passphrase of random words")]
    Phrase {
        #[arg(long, short, help = "Length of phrase (in words)", default_value_t = 4, value_parser = greater_than_zero)]
        length: u32,

        #[arg(long, short, help = "Word separator", default_value_t = String::from(" "))]
        separator: String,
    },
}
