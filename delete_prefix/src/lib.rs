pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    // if s.starts_with(prefix) {
    //     Some(&s[prefix.len()..])
    // } else {
    //     None
    // }
    s.strip_prefix(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prefix() {
        let result = delete_prefix("foo", "foobar");
        assert_eq!(result, Some("bar"));
    }
}