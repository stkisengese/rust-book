pub fn add_curry(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

pub fn twice<T>(f: T) -> impl Fn(i32) -> i32
where
    T: Fn(i32) -> i32,
{
    move |x| f(f(x))
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
            let add5 = add_curry(5);
            let add10_twice = twice(add5);
            assert_eq!(add10_twice(3), 13);
        }
    }

    #[test]
    fn test_add_curry_zero() {
        let add0 = add_curry(0);
        assert_eq!(add0(10), 10);
        assert_eq!(add0(-5), -5);
        assert_eq!(add0(0), 0);
    }
