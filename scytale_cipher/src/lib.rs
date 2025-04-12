pub fn scytale_cipher(message: String, i: u32) -> String {
    if message.is_empty() || i == 0 {
        return String::new();
    }

    let cols = i as usize;
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + cols - 1) / cols; // round up to fit all characters
    let padded_len = rows * cols;

    // pad with spaces if needed
    let mut padded_chars = chars;
    padded_chars.resize(padded_len, ' ');

    let mut result = String::with_capacity(padded_len);

    // read column-wise (simulate wrap-around reading)
    for col in 0..cols {
        for row in 0..rows {
            let index = row * cols + col;
            result.push(padded_chars[index]);
        }
    }

    result.trim().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = scytale_cipher(String::from("scytale Code"), 6);
        let result2 = scytale_cipher(String::from("scytale Code"), 8);
        assert_eq!(result, "sec yCtoadle");
        assert_eq!(result2, "sCcoydtea l e");
    }
}
