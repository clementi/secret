pub fn generate_token(length: u32, chars: &Vec<char>) -> String {
    let mut token_chars = vec![];

    for _ in 0..length {
        let index = rand::random::<usize>() % chars.len();
        let char = chars[index];
        token_chars.push(char);
    }

    String::from_iter(token_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_token_generates_a_token() {
        let length = 10;
        let token = generate_token(length, &vec!['a', 'b', 'c']);
        assert!(token.len() as u32 == length);
    }
}
