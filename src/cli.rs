use clap::{Parser, Subcommand};

fn parse_natural_number(v: &str) -> Result<u32, String> {
    let num: u32 = v
        .parse::<u32>()
        .map_err(|_| String::from("--number must be an integer greater than zero"))?;
    if num > 0 {
        Ok(num)
    } else {
        Err(String::from(
            "--number must be an integer greater than zero",
        ))
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
        value_parser = parse_natural_number
    )]
    pub count: u32,
}
#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(
        about = "Generate a token (string) of random characters",
        visible_alias = "t"
    )]
    Token {
        #[arg(long, short, help = "Length of token", default_value_t = 20, value_parser = parse_natural_number)]
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
    #[command(about = "Generate a passphrase of random words", visible_alias = "p")]
    Phrase {
        #[arg(long, short, help = "Length of phrase (in words)", default_value_t = 4, value_parser = parse_natural_number)]
        length: u32,

        #[arg(long, short, help = "Word separator", default_value_t = String::from(" "))]
        separator: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_natural_number_fails_on_non_numbers() {
        let value = "q";
        match parse_natural_number(value) {
            Ok(v) => assert!(false, "should fail on {value}, got {v}"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn parse_natural_number_succeeds_on_positive_integers() {
        let value = "1";
        match parse_natural_number(value) {
            Ok(v) => assert_eq!(v, 1),
            Err(msg) => assert!(false, "{}", msg),
        }
    }

    #[test]
    fn parse_natural_number_fails_on_nonpositive_integers() {
        let value = "0";
        match parse_natural_number(value) {
            Ok(v) => assert!(false, "should fail on {value}, got {v}"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn parse_natural_number_fails_on_nonintegral_numbers() {
        let value = "1.1";
        match parse_natural_number(value) {
            Ok(v) => assert!(false, "should fail on {value}, got {v}"),
            Err(_) => assert!(true),
        }
    }
}
