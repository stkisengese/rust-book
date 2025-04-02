pub fn sum(a: u8, b: u8) -> Result<u8, &'static str> {
    a.checked_add(b).ok_or("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> Result<i16, &'static str> {
    a.checked_sub(b).ok_or("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> Result<i8, &'static str> {
    a.checked_mul(b).ok_or("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("ERROR: attempt to divide by zero")
    } else {
        Ok(a / b)
    }
}

pub fn rem(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("ERROR: attempt to calculate remainder with zero divisor")
    } else {
        Ok(a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(254, 2), Err("ERROR: attempt to add with overflow"));
        assert_eq!(sum(1, 2), Ok(3));
    }

    #[test]
    fn diff_test() {
        assert_eq!(diff(234, 2), Ok(232));
        assert_eq!(diff(-32768, 32766), Err("ERROR: attempt to subtract with overflow"));
    }

    #[test]
    fn pro_test() {
        assert_eq!(pro(23, 2), Ok(46));
        assert_eq!(pro(-128, 2), Err("ERROR: attempt to multiply with overflow"));
    }

    #[test]
    fn quo_test() {
        assert_eq!(quo(22.0, 2.0), Ok(11.0));
        assert_eq!(quo(-128.23, 2.0), Ok(-64.115));
        assert_eq!(quo(22.0, 0.0), Err("ERROR: attempt to divide by zero"));
    }

    #[test]
    fn rem_test() {
        assert_eq!(rem(22.0, 2.0), Ok(0.0));
        assert_eq!(rem(-128.23, 2.0), Ok(-0.22999573));
        assert_eq!(rem(22.0, 0.0), Err("ERROR: attempt to calculate remainder with zero divisor"));
    }
}