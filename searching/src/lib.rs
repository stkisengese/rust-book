/// Searches for a key in an array and returns its index if found.
///
/// This function iterates through the array and returns the index of the first
/// element that matches the key. If the key is not found, it returns None.
///
/// # Arguments
///
/// * `array` - A slice of i32 values to search through
/// * `key` - The i32 value to search for
///
/// # Returns
///
/// * `Some(usize)` - The index of the matching element if found
/// * `None` - If the key is not found in the array
///
/// # Examples
///
/// ```
/// use searching::search;
///
/// let array = [1, 3, 4, 6, 8, 9, 11];
/// assert_eq!(search(&array, 6), Some(3));
/// assert_eq!(search(&array, 5), None);
/// ```
pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (index, &value) in array.iter().enumerate() {
        if value == key {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11];
        let result = search(&ar, 6);
        assert_eq!(result, Some(3));
    }
}
