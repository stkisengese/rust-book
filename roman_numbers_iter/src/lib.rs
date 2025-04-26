pub mod roman_digit {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum RomanDigit {
	    Nulla,
	    I,
	    V,
	    X,
	    L,
	    C,
	    D,
	    M,
    }

    impl From<u32> for RomanDigit {
        fn from(value: u32) -> Self {
            match value {
                0 => RomanDigit::Nulla,
                1 => RomanDigit::I,
                5 => RomanDigit::V,
                10 => RomanDigit::X,
                50 => RomanDigit::L,
                100 => RomanDigit::C,
                500 => RomanDigit::D,
                1000 => RomanDigit::M,
                _ => panic!("Invalid value"),
            }
        }
    }
}

pub mod roman_number {
    use super::roman_digit::RomanDigit;
    use RomanDigit::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RomanNumber(pub Vec<RomanDigit>);

    
    impl From<u32> for RomanNumber {
        fn from(value: u32) -> Self {
            if value == 0 {
                return RomanNumber(vec![RomanDigit::Nulla]);
            }

            let mut value1 = value;
            
            let mut digits = Vec::new();
            let mapping = [
                (1000, vec![M]),
                (900, vec![C, M]),
                (500, vec![D]),
                (400, vec![C, D]),
                (100, vec![C]),
                (90, vec![X, C]),
                (50, vec![L]),
                (40, vec![X, L]),
                (10, vec![X]),
                (9, vec![I, X]),
                (5, vec![V]),
                (4, vec![I, V]),
                (1, vec![I]),
            ];

            for &(num, ref roman) in mapping.iter() {
                while *&value1 >= num {
                    digits.extend(roman.clone());
                    value1 -= num;
                }
            }

            RomanNumber(digits)
        }
    }

    impl Iterator for RomanNumber {
        type Item = RomanNumber;
        
        fn next(&mut self) -> Option<Self::Item> {
            let mut next_digit = self.0.clone();
            next_digit.push(RomanDigit::I);
            Some(RomanNumber(next_digit))
        }
    }
}

pub use roman_digit::RomanDigit;
pub use roman_number::RomanNumber;


#[cfg(test)]
mod tests {
    use super::roman_digit::RomanDigit::*;
    use super::roman_number::RomanNumber;

    #[test]
    fn test_roman_number() {
        let result = RomanNumber::from(1234);
        assert_eq!(result, RomanNumber(vec![M, C, C, X, X, X, I, V]));

        assert_eq!(RomanNumber::from(0), RomanNumber(vec![Nulla]));
        assert_eq!(RomanNumber::from(9), RomanNumber(vec![I, X]));
        assert_eq!(RomanNumber::from(45), RomanNumber(vec![X, L, V]));
        assert_eq!(RomanNumber::from(32), RomanNumber(vec![X, X, X,I, I]));
    }

    #[test]
    fn test_roman_number_iterator() {
        let mut number = RomanNumber::from(15);
        assert_eq!(number, RomanNumber(vec![X, V]));
        assert_eq!(number.next(), Some(RomanNumber(vec![X, V, I])));
    }
}
