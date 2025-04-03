pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = str_len("olá!");
        assert_eq!(result, 4);
    }
}
