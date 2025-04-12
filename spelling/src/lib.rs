/// Instructions

/// In this exercise, you'll create the function spell that will spell a generated number.

/// Here are some examples of what your function should return:

///     1 -> "one"
///     14 -> "fourteen".
///     96 -> "ninety-six"
///    100 -> "one hundred".
///     101 -> "one hundred one"
///     348 -> "three hundred forty-eight"
///     1002 -> "one thousand two".
///     1000000 -> "one million"

///     Only positive numbers will be tested, up to "one million".

pub fn spell(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=9 => spell_units(n),
        10..=19 => spell_teens(n),
        20..=99 => spell_tens(n),
        100..=999 => spell_hundreds(n),
        1000..=999999 => spell_thousands(n),
        1000000..=1000000 => "one million".to_string(),
        _ => String::from("Number too large"),
    }
}
fn spell_units(n: u64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        _ => String::from(""),
    }
}

fn spell_teens(n: u64) -> String {
    match n {
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        _ => String::from(""),
    }
}

fn spell_tens(n: u64) -> String {
    let tens_digit = n / 10;
    let units_digit = n % 10;
    
    let tens_word = match tens_digit {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "", 
    };
    
    match units_digit {
        0 => String::from(tens_word),
        _ => format!("{}-{}", tens_word, spell_units(units_digit)),
    }
}
fn spell_hundreds(n: u64) -> String {
    let hundreds_digit = n / 100;
    let remainder = n % 100;
    
    match remainder {
        0 => format!("{} hundred", spell_units(hundreds_digit)),
        _ => format!("{} hundred {}", spell_units(hundreds_digit), spell(remainder)),
    }
}

fn spell_thousands(n: u64) -> String {
    let thousands = n / 1000;
    let remainder = n % 1000;
    
    match remainder {
        0 => format!("{} thousand", spell(thousands)),
        _ => format!("{} thousand {}", spell(thousands), spell(remainder)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell() {   
        assert_eq!(spell(348), "three hundred forty-eight");
        assert_eq!(spell(1000000), "one million");
        assert_eq!(spell(101), "one hundred one");
        assert_eq!(spell(999999), "nine hundred ninety-nine thousand nine hundred ninety-nine");
        assert_eq!(spell(0), "zero");
    }
}
