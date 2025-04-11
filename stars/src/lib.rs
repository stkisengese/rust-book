// Instructions

// Create a function named stars that takes a u32 as an argument 
// and returns a String of stars (asterisks). The number of stars 
//returned is 2^n (2 to the nth power).

pub fn stars(n: u32) -> String {
    "*".repeat(2u32.pow(n).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = stars(1);
        assert_eq!(result, "**");
    }

    #[test]
    fn it_works2() {
        let result = stars(4);
        assert_eq!(result, "****************");
    }
}
