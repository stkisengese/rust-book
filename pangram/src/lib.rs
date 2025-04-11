// Instructions

// Create a function named is_pangram which returns true if the given string is a pangram.

// A pangram is a sentence which uses every letter of the alphabet at least once.

// Example: "The quick brown fox jumps over the lazy dog."

pub fn is_pangram(s: &str) -> bool {
    let lowercase_s = s.to_lowercase();
    ('a'..='z').all(|c| lowercase_s.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_pangram("this is not a pangram!");
        assert_eq!(result, false);
    }
}
