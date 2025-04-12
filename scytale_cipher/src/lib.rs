pub fn scytale_cipher(message: String, i: u32) -> String {
    let size = i as usize;
    if message.is_empty() || i == 0 {
        return String::new();
    }

    let num_colums = (message.len() + size -1) / size;
    let mut result = String::with_capacity(message.len());

    for col in 0..size {
        for row in 0..num_colums {
            let index = row * size + col;
            if index < message.len() {
                result.push(message.chars().nth(index).unwrap());
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
        assert_eq!(result2, "sCcoydtea l e");
    }
}
