pub fn char_length(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_length() {
        let result = char_length("hello");
        assert_eq!(result, 5);
    }
}
