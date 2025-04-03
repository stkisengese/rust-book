// is_empty: that returns true if the string is empty.
pub fn is_empty(v: &str) -> bool {
    v.is_empty()
    // v.len() == 0
    // v == ""
}

// is_ascii: that returns true if all characters are within the ASCII range.
pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

// contains: that returns true if the string contains the given character.
pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

// split_at: that divides a string in two returning a tuple.
pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

//find: that returns the index of the first character of a given string that matches the pattern.
pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(v.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_test() {
        let result = is_empty("Hello, world!");
        assert_eq!(result, false);
    }

    #[test]
    fn is_ascii_test() {
        let result = is_ascii("Hello, world!");
        assert_eq!(result, true);
    }

    #[test]
    fn contains_test() {
        let result = contains("Hello, world!", "world");
        assert_eq!(result, true);
    }

    #[test]
    fn split_at_test() {
        let result = split_at("Hello, world!", 5);
        assert_eq!(result, ("Hello", ", world!"));
    }

    #[test]
    fn find_test() {
        let result = find("rust", 'u');
        assert_eq!(result, 1);
    }
}
