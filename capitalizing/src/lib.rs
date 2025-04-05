// capitalize_first function which converts the 
// first letter of the string to uppercase.
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}


// title_case function which converts the 
// first letter of each word in a string to uppercase.

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

// change_case function which converts all uppercase letters to lowercase and vice versa.
pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        let result = capitalize_first("hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_title_case() {
        let result = title_case("hello world");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_change_case() {
        let result = change_case("hElLo wOrLd");
        assert_eq!(result, "HeLlO WoRlD");
    }
}
