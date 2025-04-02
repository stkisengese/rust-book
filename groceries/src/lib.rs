pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut vec = Vec::new();
        insert(&mut vec, "apple".to_string());
        assert_eq!(vec[0], "apple");
    }
    #[test]
    fn at_index_test() {
        let vec = vec!["apple".to_string(), "banana".to_string()];
        let result = at_index(&vec, 1);
        assert_eq!(result, "banana");
    }
}
