pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + key.rem_euclid(26)as u8)%26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + key.rem_euclid(26)as u8)%26) as char,
        _ => c as char
    }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13);
        let result2 =  rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5);
        let result3 = rotate("Testing", -14);
        let result4 = rotate("a", -1);
        assert_eq!(result, "The five boxing wizards jump quickly.");
        assert_eq!(result2, "Ryg aesmuvi nkpd tewzsxq jolbkc foh");
        assert_eq!(result3, "Fqefuzs");
        assert_eq!(result4, "z");
    }
}
