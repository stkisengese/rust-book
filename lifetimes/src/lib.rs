#[derive(Debug, Clone)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
	pub fn new(name: &'a str) -> Person<'a> {
        Person { 
            name,
            age: 0,
        }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let person = Person::new("John");
        assert_eq!(person.name, "John");
        assert_eq!(person.age, 0);
    }
}