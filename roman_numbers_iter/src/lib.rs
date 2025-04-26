use crate::RomanDigit::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
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

        let mut digits = Vec::new();
        let mut value1 = value;

        if value == 0 {
            digits.push(Nulla);
        } else {
            for &(num, ref roman) in mapping.iter() {
                while value1 >= num {
                    digits.extend(roman.clone());
                    value1 -= num;
                }
            }
        }

        RomanNumber(digits)
    }
}

// Implement the Iterator trait for RomanNumber
impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment the value represented by the Roman numeral
        let current_value = RomanNumber::to_u32(&self.0) + 1;

        // Update the digits to reflect the new value
        let next_roman = RomanNumber::from(current_value);
        self.0 = next_roman.0.clone();

        Some(RomanNumber(self.0.clone()))
    }
}

impl RomanNumber {
    // Helper function to convert RomanNumber back to u32
    fn to_u32(digits: &[RomanDigit]) -> u32 {
        let mapping = [
            (M, 1000),
            (D, 500),
            (C, 100),
            (L, 50),
            (X, 10),
            (V, 5),
            (I, 1),
            (Nulla, 0),
        ];

        let mut total = 0;
        let mut prev_value = 0;

        for digit in digits.iter().rev() {
            let value = mapping.iter().find(|(d, _)| *d == *digit).unwrap().1;

            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }

            prev_value = value;
        }

        total
    }
}

#[cfg(test)]
mod tests {
   use super::*;

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
        let mut number = RomanNumber::from(8);
        assert_eq!(number, RomanNumber(vec![V, I, I, I]));
        assert_eq!(number.next(), Some(RomanNumber(vec![I, X])));
    }
}
