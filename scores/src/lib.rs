// Instructions

// Let's play a little.

// Create a function named score that given a &str, computes the score for that given string as a u64.

// Each letter has a value, you just have to sum the values of the letters in the given string.

// You will need these:
// Letter 	Value
// A, E, I, O, U, L, N, R, S, T 	1
// D, G 	2
// B, C, M, P 	3
// F, H, V, W, Y 	4
// K 	5
// J, X 	8
// Q, Z 	10

pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0, // Any other characters are worth 0 points
        }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = score("ThiS is A Test");
        assert_eq!(result, 14);
    }
}
