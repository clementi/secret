use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

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
