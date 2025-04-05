// use arrays::sum;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2
    } else {
        sorted[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }
    let (&mode, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        let result = mean(&v);
        assert_eq!(result, 3.857142857142857
        );
    }
    #[test]
    fn test_median() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        let result = median(&v);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_mode() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        let result = mode(&v);
        assert_eq!(result, 5);
    }
}
