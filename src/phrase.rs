pub fn generate_phrase(
    length: u32,
    separator: &String,
    word_list: &Vec<String>,
) -> String {
    let mut words = vec![];

    for _ in 0..length {
        let word = get_word(word_list);
        words.push(word);
    }

    words.join(separator)
}

fn get_word(word_list: &Vec<String>) -> &str {
    // let index = get_index();
    let index = rand::random::<usize>() % word_list.len();
    word_list[index].as_str()
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::word_list;

    use super::*;

    #[test]
    fn generate_phrase_generates_a_phrase() {
        let length = 2;
        match word_list::load_word_list() {
            Ok(word_list) => {
                let separator = String::from("-");

                let phrase = generate_phrase(length, &separator, &word_list);
                let parts: Vec<String> = phrase.split("-").map(|s| s.to_string()).collect();
                let words: HashSet<String> = HashSet::from_iter(word_list.iter().cloned());

                assert!(word_list.contains(&parts[0]) && words.contains(&parts[1]));
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_word_returns_a_word_from_the_word_list() {
        match word_list::load_word_list() {
            Ok(word_list) => {
                let word = String::from(get_word(&word_list));
                assert!(word_list
                    .contains(&word))
            }
            Err(_) => assert!(false),
        }
    }
}
