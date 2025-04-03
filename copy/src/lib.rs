// Instructions

// Create the following functions. The objective is to know how ownership works with different types.

//     nbr_function returns a tuple:
//         with the original value.
//         the exponential function of the value.
//         and the natural logarithm of the absolute value.
//     str_function returns a tuple:
//         with the original value.
//         and the exponential function of each value as a string (see the example).
//     vec_function returns a tuple:
//         with the original value.
//         and the natural logarithm of each absolute value.


pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exponential = (c as f64).exp();
    let natural_logarithm = (c.abs() as f64).ln();
    (c, exponential, natural_logarithm)
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<f64> = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    
    let exp_numbers: Vec<String> = numbers
        .iter()
        .map(|&n| n.exp().to_string())
        .collect();
    
    (a, exp_numbers.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b
        .iter()
        .map(|&n| (n.abs() as f64).ln())
        .collect();
    
    (b, ln_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        let inf = f64::NEG_INFINITY;
        let result = nbr_function(0);
        assert_eq!(result, (0, 1.0, inf));
        // let result = str_function("1 2 4 5 6".to_string());
        // assert_eq!(result, ("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351"));
        // let result = vec_function(vec![1, 2, 3, 4, 5]);
        // assert_eq!(result, ([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]));
    }
}
