use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

const INDEX_LENGTH: u32 = 5;
const DIE_SIZE: u32 = 6;

pub fn load_dictionary() -> HashMap<String, String> {
    let file = File::open("./eff_large_wordlist.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut dictionary = HashMap::new();

    for result in lines {
        let line = result.unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        dictionary.insert(String::from(parts[0]), String::from(parts[1]));
    }

    dictionary
}

pub fn get_word(dictionary: &HashMap<String, String>) -> &str {
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
