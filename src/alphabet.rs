use std::fmt;

pub struct AlphabetOptions {
    pub alpha_lower: bool,
    pub alpha_upper: bool,
    pub alpha: bool,
    pub numeric: bool,
    pub alphanumeric: bool,
    pub symbols: bool,
    pub all: bool,
}

impl AlphabetOptions {
    pub fn is_valid(&self) -> bool {
        self.alpha_lower
            || self.alpha_upper
            || self.alpha
            || self.numeric
            || self.alphanumeric
            || self.symbols
            || self.all
    }
}

#[derive(Debug, Clone)]
pub struct InvalidOptionsError;

impl fmt::Display for InvalidOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid options")
    }
}

impl std::error::Error for InvalidOptionsError {}

const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const NUMERIC: &str = "0123456789";

const SYMBOLS: &str = "`~!@#$%^&*()_+-={}|[]\\:;'\"<>?,./";

pub fn get_alphabet(options: &AlphabetOptions) -> String {
    let mut alphabet = String::from("");

    let alpha_lower = options.alpha_lower || options.alpha || options.alphanumeric || options.all;
    let alpha_upper = options.alpha_upper || options.alpha || options.alphanumeric || options.all;
    let numeric = options.numeric || options.alphanumeric || options.all;
    let symbols = options.symbols || options.all;

    if alpha_lower {
        alphabet.push_str(ALPHA_LOWER);
    }
    if alpha_upper {
        alphabet.push_str(ALPHA_UPPER);
    }
    if numeric {
        alphabet.push_str(NUMERIC);
    }
    if symbols {
        alphabet.push_str(SYMBOLS);
    }

    alphabet
}
