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
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.extend(c.to_lowercase());
        }
    }
    result
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
        let result = title_case("hello my\t\tname is carl");
        assert_eq!(result, "Hello My\t\tName Is Carl");
    }

    #[test]
    fn test_change_case() {
        let result = change_case("hElLo wOrLd");
        assert_eq!(result, "HeLlO WoRlD");
    }
}
