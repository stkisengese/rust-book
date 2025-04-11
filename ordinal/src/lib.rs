/// Converts a cardinal number to its ordinal form.
///
/// # Examples:
/// - 1 -> "1st"
/// - 2 -> "2nd"
/// - 3 -> "3rd"
/// - 4 -> "4th"
/// - 11 -> "11th"
/// - 21 -> "21st"
///
/// # Arguments
///
/// * `x` - A cardinal number to convert
///
/// # Returns
///
/// The ordinal representation of the number as a String
pub fn num_to_ordinal(x: u32) -> String {
    let suffix = if (11..=13).contains(&(x % 100)) {
        // Special case for 11th, 12th, 13th
        "th"
    } else {
        match x % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };

    format!("{}{}", x, suffix)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = num_to_ordinal(12);
        assert_eq!(result, "12th");
    }
}
