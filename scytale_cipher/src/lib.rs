pub fn scytale_cipher(message: String, i: u32) -> String {
    let size = i as usize;
    if message.is_empty() || i == 0 {
        return String::new();
    }

    let chars: Vec<char> = message.chars().collect(); // only once!
    let num_columns = (chars.len() + size - 1) / size; // ceiling division
    let mut result = String::with_capacity(chars.len());

    for col in 0..size {
        for row in 0..num_columns {
            let index = row * size + col;
            if index < chars.len() {
                result.push(chars[index]);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = scytale_cipher(String::from("scytale Code"), 6);
        let result2 = scytale_cipher(String::from("scytale Code"), 8);
        assert_eq!(result, "sec yCtoadle");
        assert_eq!(result2, "sCcoydteale ");
    }
}
