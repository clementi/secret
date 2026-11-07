use std::error::Error;

pub fn load_word_list() -> Result<Vec<String>, Box<dyn Error>> {
    let word_list_contents = include_str!("../inc/eff_large_wordlist.txt");
    let lines = word_list_contents.lines();

    let mut word_list = vec![];

    for line in lines {
        word_list.push(String::from(line));
    }

    Ok(word_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_dictionary_loads_the_dictionary() {
        match load_word_list() {
            Ok(dictionary) => assert!(!dictionary.is_empty()),
            Err(msg) => assert!(false, "error creating dictionary: {msg}"),
        }
    }
}
