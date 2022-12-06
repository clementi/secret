use std::{collections::HashMap, error::Error};

pub fn load_dictionary() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let dictionary_contents = include_str!("../inc/eff_large_wordlist.txt");
    let lines = dictionary_contents.lines();

    let mut dictionary = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split('\t').collect();
        dictionary.insert(String::from(parts[0]), String::from(parts[1]));
    }

    Ok(dictionary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_dictionary_loads_the_dictionary() {
        match load_dictionary() {
            Ok(dictionary) => assert!(!dictionary.is_empty()),
            Err(msg) => assert!(false, "error creating dictionary: {msg}"),
        }
    }
}
