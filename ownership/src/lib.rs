/// Returns the first sub-word from a string in camelCase, PascalCase, or snake_case format
/// 
/// # Arguments
/// * `s` - A string to extract the first sub-word from
/// 
/// # Examples
/// ```
// let result = first_subword("HelloWorld".to_string()); // Returns "Hello"
// let result = first_subword("hello_world".to_string()); // Returns "hello"
// let result = first_subword("camelCase".to_string()); // Returns "camel"
// ```
pub fn first_subword(mut s: String) -> String {
    // Skip the first character when checking for boundaries
    let boundary = s.char_indices()
        .skip(1)  // ← Key change: ignore first character
        .find(|(_, c)| *c == '_' || c.is_uppercase())  // ← Key change: dereference the char
        .map(|(i, _)| i)
        .unwrap_or_else(|| s.len());
    
    s.truncate(boundary);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_subword_test() {
        assert_eq!(first_subword("HelloWorld".to_owned()), "Hello");
        assert_eq!(first_subword("camelCase".to_owned()), "camel");
        assert_eq!(first_subword("snake_case".to_owned()), "snake");
    }
}