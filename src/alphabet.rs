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
    pub fn all_false(&self) -> bool {
        !self.alpha_lower
            && !self.alpha_upper
            && !self.alpha
            && !self.numeric
            && !self.alphanumeric
            && !self.symbols
            && !self.all
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alphabet_gets_an_alphabet() {
        for bits in 0..127 {
            let options = build_options(bits);
            let alphabet = get_alphabet(&options);

            if has_alpha_lower(&options) {
                assert!(alphabet.contains(ALPHA_LOWER))
            }

            if has_alpha_upper(&options) {
                assert!(alphabet.contains(ALPHA_UPPER))
            }

            if has_numeric(&options) {
                assert!(alphabet.contains(NUMERIC))
            }

            if has_symbols(&options) {
                assert!(alphabet.contains(SYMBOLS))
            }
        }
    }

    fn build_options(bits: u8) -> AlphabetOptions {
        AlphabetOptions {
            alpha_lower: bits & 0x40 != 0,
            alpha_upper: bits & 0x20 != 0,
            alpha: bits & 0x10 != 0,
            numeric: bits & 0x08 != 0,
            alphanumeric: bits & 0x04 != 0,
            symbols: bits & 0x02 != 0,
            all: bits & 0x01 != 0,
        }
    }

    fn has_alpha_lower(options: &AlphabetOptions) -> bool {
        options.all || options.alpha_lower || options.alpha || options.alphanumeric
    }

    fn has_alpha_upper(options: &AlphabetOptions) -> bool {
        options.all || options.alpha_upper || options.alpha || options.alphanumeric
    }

    fn has_numeric(options: &AlphabetOptions) -> bool {
        options.all || options.numeric || options.alphanumeric
    }

    fn has_symbols(options: &AlphabetOptions) -> bool {
        options.all || options.symbols
    }
}
