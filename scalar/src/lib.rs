use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ScalarError {
    Overflow,
    DivisionByZero,
}

impl fmt::Display for ScalarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScalarError::Overflow => write!(f, "ERROR: attempt to add with overflow"),
            ScalarError::DivisionByZero => write!(f, "ERROR: attempt to divide by zero"),
        }
    }
}

pub fn sum(a: u8, b: u8) -> Result<u8, ScalarError> {
    a.checked_add(b).ok_or(ScalarError::Overflow)
}

pub fn diff(a: i16, b: i16) -> Result<i16, ScalarError> {
    a.checked_sub(b).ok_or(ScalarError::Overflow)
}

pub fn pro(a: i8, b: i8) -> Result<i8, ScalarError> {
    a.checked_mul(b).ok_or(ScalarError::Overflow)
}

pub fn quo(a: f32, b: f32) -> Result<f32, ScalarError> {
    if b == 0.0 {
        Err(ScalarError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn rem(a: f32, b: f32) -> Result<f32, ScalarError> {
    if b == 0.0 {
        Err(ScalarError::DivisionByZero)
    } else {
        Ok(a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(254, 2), Err(ScalarError::Overflow));
        assert_eq!(sum(1, 2), Ok(3));
    }

    #[test]
    fn diff_test() {
        assert_eq!(diff(234, 2), Ok(232));
        assert_eq!(diff(-32768, 32766), Err(ScalarError::Overflow));
    }

    #[test]
    fn pro_test() {
        assert_eq!(pro(23, 2), Ok(46));
        assert_eq!(pro(-128, 2), Err(ScalarError::Overflow));
    }

    #[test]
    fn quo_test() {
        assert_eq!(quo(22.0, 2.0), Ok(11.0));
        assert_eq!(quo(-128.23, 2.0), Ok(-64.115));
        assert_eq!(quo(22.0, 0.0), Err(ScalarError::DivisionByZero));
    }

    #[test]
    fn rem_test() {
        assert_eq!(rem(22.0, 2.0), Ok(0.0));
        assert_eq!(rem(-128.23, 2.0), Ok(-0.22999573));
        assert_eq!(rem(22.0, 0.0), Err(ScalarError::DivisionByZero));
    }
}
