pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;
    let sum = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>();
    num == sum
}


// pub fn number_logic(num: u32) -> bool {
//     let digits: Vec<u32> = num
//         .to_string()
//         .chars()
//         .map(|c| c.to_digit(10).unwrap())
//         .collect();

//     let len = digits.len() as u32;

//     num == digits.iter().map(|d| d.pow(len)).sum()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = number_logic(9);
        assert_eq!(result, true);
    }
}
