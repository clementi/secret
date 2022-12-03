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
