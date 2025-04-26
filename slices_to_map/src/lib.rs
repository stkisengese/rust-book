pub use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    // Determine the minimum length between the two slices
    let min_length = keys.len().min(values.len());

    // Create a new HashMap
    let mut map = HashMap::new();

    // Iterate over the slices up to the minimum length
    for i in 0..min_length {
        map.insert(&keys[i], &values[i]);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_equal_length_slices() {
        let keys = ["Olivia", "Liam", "Emma"];
        let values = [1, 3, 23];
        let result = slices_to_map(&keys, &values);

        let mut expected = HashMap::new();
        expected.insert(&"Olivia", &1);
        expected.insert(&"Liam", &3);
        expected.insert(&"Emma", &23);

        assert_eq!(result, expected);
    }

}
