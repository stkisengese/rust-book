// Instructions

// Create a function named initials. This function will receive a vector
//  of string literals with names, and return a vector of Strings with the initials of each name.

pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|name| {
        let mut initials = String::new();
        for word in name.split_whitespace() {
            if let Some(c) = word.chars().next() {
                initials.push(c);
                initials.push_str(". ");
                // initials.make_ascii_uppercase();
            }
        }
        initials.pop();
        initials
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names);
        assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
