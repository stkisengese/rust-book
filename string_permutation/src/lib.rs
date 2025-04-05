// use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut v1: Vec<char> = s1.chars().collect();
    let mut v2: Vec<char> = s2.chars().collect();
    v1.sort();
    v2.sort();
    v1 == v2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        let result = is_permutation("hello", "olleh");
        assert_eq!(result, true);
    }
}
