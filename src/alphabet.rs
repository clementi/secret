pub const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

pub const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub const NUMERIC: &str = "0123456789";

pub const SYMBOLS: &str = "`~!@#$%^&*()_+-={}|[]\\:;'\"<>?,./";

pub fn get_alphabet(sets: &Vec<String>) -> String {
    let mut alphabets = vec![];

    for set in sets.iter() {
        if set == "alpha-lower" {
            alphabets.push(ALPHA_LOWER);
        } else if set == "alpha-upper" {
            alphabets.push(ALPHA_UPPER);
        } else if set == "numeric" {
            alphabets.push(NUMERIC);
        } else if set == "symbols" {
            alphabets.push(SYMBOLS);
        }
    }

    if sets.is_empty() {
        alphabets.push(ALPHA_LOWER);
        alphabets.push(ALPHA_UPPER);
        alphabets.push(NUMERIC);
        alphabets.push(SYMBOLS);
    }

    alphabets.join("")
}
