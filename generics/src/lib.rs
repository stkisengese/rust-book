pub fn identity<T>(v: T) -> T {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = identity("Hello, world!");
        assert_eq!(result, "Hello, world!");
        assert_eq!(identity(3), 3);
    }
}
