use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

pub fn load_dictionary() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let file = File::open("./eff_large_wordlist.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut dictionary = HashMap::new();

    for result in lines {
        let line = result.unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        dictionary.insert(String::from(parts[0]), String::from(parts[1]));
    }

    Ok(dictionary)
}
