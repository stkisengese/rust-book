pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.is_empty() {
        return Vec::new();
    }
    if arr.len() == 1 {
        return vec![0];
    }
    if arr.len() == 2 {
        return vec![arr[1], arr[0]];
    }
    
    arr.iter()
       .enumerate()
       .map(|(i, _)| {
           arr.iter()
              .enumerate()
              .filter(|&(j, _)| j != i)
              .map(|(_, &x)| x)
              .product()
       })
       .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_products() {
        let arr = vec![1, 7, 3, 4];
        let expected = vec![84, 12, 28, 21];
        assert_eq!(get_products(arr), expected);
    }
}
