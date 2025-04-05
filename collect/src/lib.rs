

pub fn bubble_sort(arr: &mut [i32]) {
    arr.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [1, 2, 3, 4];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4]);
    }
}
