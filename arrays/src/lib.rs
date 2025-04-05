pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let a = [1, 2, 3];
        let result = sum(&a);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_thirtytwo_tens() {
        let result = thirtytwo_tens();
        assert_eq!(result.len(), 32);
        assert_eq!(result.iter().sum::<i32>(), 320);
    }
}
