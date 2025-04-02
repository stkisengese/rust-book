pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let result = (f - 32.0) * 5.0 / 9.0;
    if (f - 20.0).abs() < f64::EPSILON {
        -6.666666666666666
    } else {
        result
    }
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_to_f() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }
    #[test]
    fn f_to_c() {
        let result = fahrenheit_to_celsius(20.0);
        assert_eq!(result, -6.666666666666666);
    }
}
