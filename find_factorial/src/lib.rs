pub fn factorial(num: u64) -> u64 {
    if num <= 1 {
        1
    } else {
        num * factorial(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_test() {
        let result = factorial(0);
        assert_eq!(result, 1);
    }
}
