pub fn generate_token(length: u32, chars: &Vec<char>) -> String {
    let mut token_chars = vec![];

    for _ in 0..length {
        let index = rand::random::<usize>() % chars.len();
        let char = chars[index];
        token_chars.push(char);
    }

    String::from_iter(token_chars)
}
