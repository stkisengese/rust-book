pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_curry_positive() {
        let add5 = add_curry(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-5), 0);
    }

    #[test]
    fn test_add_curry_negative() {
        let sub3 = add_curry(-3);
        assert_eq!(sub3(10), 7);
        assert_eq!(sub3(0), -3);
        assert_eq!(sub3(3), 0);
    }

    #[test]
    fn test_add_curry_zero() {
        let add0 = add_curry(0);
        assert_eq!(add0(10), 10);
        assert_eq!(add0(-5), -5);
        assert_eq!(add0(0), 0);
    }

    #[test]
    fn test_add_curry_large_numbers() {
        let add_million = add_curry(1_000_000);
        assert_eq!(add_million(1), 1_000_001);
        assert_eq!(add_million(-1_000_000), 0);
    }

    #[test]
    fn test_multiple_instances() {
        let add10 = add_curry(10);
        let add20 = add_curry(20);
        
        assert_eq!(add10(5), 15);
        assert_eq!(add20(5), 25);
        assert_eq!(add10(add20(5)), 35); // Chaining
    }

    #[test]
    fn test_closure_reuse() {
        let add7 = add_curry(7);
        assert_eq!(add7(3), 10);
        assert_eq!(add7(10), 17); // Same closure reused
    }
}