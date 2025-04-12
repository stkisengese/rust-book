pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let text_chars: Vec<char> = text.chars().collect();
    let mut consonants = Vec::new();
    let len = text_chars.len();

    while !is_vowel(text_chars[consonants.len()]) {
        consonants.push(text_chars[consonants.len()]);

    }
    println!("consonants: {:?}", consonants.len());
    println!("text_chars: {:?}", text_chars[1]);

    if consonants.len() >= 2 && consonants[1] == 'q' && text_chars[2] == 'u' {
        consonants.push('u');
    }
    println!("consonants: {:?}", consonants);
    if consonants.is_empty() {
        return format!("{}ay", text);
    }

    let mut result = String::new();
    for i in consonants.len()..len {
        result.push(text_chars[i]);
    }
    println!("result: {:?}", result);

    for c in consonants {
        result.push(c);
    }
    result.push_str("ay");
    result
}
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pig_latin(&String::from("igloo"));
        let result2 = pig_latin(&String::from("apple"));
        let result3= pig_latin(&String::from("hello"));
        let result4 = pig_latin(&String::from("square"));
        let result5 = pig_latin(&String::from("xenon"));
        let result6 = pig_latin(&String::from("chair"));
        let result7 = pig_latin(&String::from("queen"));
    
        assert_eq!(result, "iglooay");
        assert_eq!(result2, "appleay");
        assert_eq!(result3, "ellohay");
        assert_eq!(result4, "aresquay");
        assert_eq!(result5, "enonxay");
        assert_eq!(result6, "airchay");
        assert_eq!(result7, "ueenqay");
    }
}
