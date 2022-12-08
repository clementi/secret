use std::collections::HashMap;

const INDEX_LENGTH: u32 = 5;
const DIE_SIZE: u32 = 6;

pub fn generate_phrase(
    length: u32,
    separator: &String,
    dictionary: &HashMap<String, String>,
) -> String {
    let mut words = vec![];

    for _ in 0..length {
        let word = get_word(dictionary);
        words.push(word);
    }

    words.join(separator)
}

fn get_word(dictionary: &HashMap<String, String>) -> &str {
    let index = get_index();
    dictionary.get(&index).unwrap()
}

fn get_index() -> String {
    let mut rolls = vec![];
    for _ in 0..INDEX_LENGTH {
        rolls.push(roll());
    }
    rolls.iter().map(|n| n.to_string()).collect()
}

fn roll() -> u32 {
    rand::random::<u32>() % DIE_SIZE + 1
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::dictionary;

    use super::*;

    #[test]
    fn generate_phrase_generates_a_phrase() {
        let length = 2;
        match dictionary::load_dictionary() {
            Ok(dictionary) => {
                let separator = String::from("-");

                let phrase = generate_phrase(length, &separator, &dictionary);
                let parts = phrase.split("-").collect::<Vec<&str>>();
                let words = dictionary.values().cloned().collect::<HashSet<String>>();

                assert!(words.contains(parts[0]) && words.contains(parts[1]));
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_word_returns_a_word_from_the_dictionary() {
        match dictionary::load_dictionary() {
            Ok(dictionary) => {
                let word = get_word(&dictionary);
                assert!(dictionary
                    .values()
                    .cloned()
                    .collect::<HashSet<String>>()
                    .contains(word))
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_index_returns_a_valid_index() {
        let index = get_index();
        assert!(is_valid_index(index))
    }

    fn is_valid_index(index: String) -> bool {
        index
            .chars()
            .all(|c| is_valid_digit((c as u8) - ('0' as u8)))
    }

    fn is_valid_digit(digit: u8) -> bool {
        digit <= 6 && digit >= 1
    }

    #[test]
    fn roll_returns_an_int_in_the_right_range() {
        let rolled = roll() as u8;
        assert!(is_valid_digit(rolled))
    }
}
