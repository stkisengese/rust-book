
// Instructions

// Build the function talking which will allow you to talk with your computer.

// Its answers will be created by you following the rules below.

//     It answers "There is no need to yell, calm down!" if you yell at it. For example "LEAVE ME ALONE!". Yelling is when all the letters are capital letters.
//     It answers "Sure." if you ask it something without yelling. For example "Is everything ok with you?".
//     It answers "Quiet, I am thinking!" if you yell a question at it. FOr example: "HOW ARE YOU?".
//     It says "Just say something!" if you address it without actually saying anything.
//     It answers "Interesting" to anything else.

pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let has_letters = text.chars().any(|c| c.is_alphabetic());
    
    // Check if all alphabetic characters are uppercase (yelling)
    let is_yelling = has_letters && text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
    let is_question = text.trim().ends_with('?');
    

    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = talking("Quiet, I am thinking?");
        assert_eq!(result, "Sure.");
    }
    #[test]
    fn it_works2() {
        let result = talking("something");
        assert_eq!(result, "Interesting");
    }
}
