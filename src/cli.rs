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
            help = "Use everything (equivalent to --alphanumeric --symbols)"
        )]
        all: bool,
    },
    #[command(about = "Generate a passphrase of random words")]
    Phrase {
        #[arg(long, short, help = "Length of phrase (in words)", default_value_t = 4)]
        length: u32,

        #[arg(long, short, help = "Word separator", default_value_t = String::from(" "))]
        separator: String,
    },
}

// #[derive(Debug, Clone)]
// pub enum Alphabet {
//     AlphaLower,
//     AlphaUpper,
//     Alpha,
//     Numeric,
//     AlphaNumeric,
//     Symbols,
//     All,
// }

// impl fmt::Display for Alphabet {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }
