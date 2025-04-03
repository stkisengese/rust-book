pub fn arrange_phrase(phrase: &str) -> String {
    // Split into words and collect into a vector
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    // Sort by the number found in each word
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_ascii_digit())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0)
    });
    
    // Remove numbers from words and join
    words.iter()
        .map(|word| word.chars().filter(|c| !c.is_ascii_digit()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}