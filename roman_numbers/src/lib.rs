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

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid value"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        if value == 0 {
            return RomanNumber(vec![Nulla]);
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

}
