pub fn divide(x: i32, y: i32) -> (i32, i32) {
    match y {
        0 => panic!("Division by zero"),
        _ => (x / y, x % y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_test() {
        let result = divide(9, 4);
        assert_eq!(result, (2, 1));
    }
}
