// Instructions

// Build the function make_diamond which takes a letter as an input, and returns a diamond.

// Rules:

//     The first and last row contain one 'A'.
//     The given letter has to be at the widest point.
//     All rows, except the first and last, have exactly two identical letters.
//     All rows have as many trailing spaces as leading spaces. This may be 0.
//     The diamond is vertically symmetrical, and horizontally symmetrical.
//     The width of the diamond equals its height.
//     The top half has letters in ascending order (abcd).
//     The bottom half has letters in descending order (dcba).

pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;  // 'A' is 0, 'B' is 1, etc.
    let size = 2 * n + 1;  // Total number of lines
    
    (0..size).map(|i| {
        let current_char = (b'A' + (n as i32 - (i as i32 - n as i32).abs()) as u8) as char;
        let left = (i as i32 - n as i32).abs() as usize;
        let right = size - 1 - left;
        let mut line = vec![' '; size];
        line[left] = current_char;
        if left != right {
            line[right] = current_char;
        }
        line.into_iter().collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_diamond('C');
        assert_eq!(result, ["  A  ", " B B ", "C   C", " B B ", "  A  "]);
    }
}
