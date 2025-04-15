

pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let store: Box<Vec<u32>> = Box::new(
        s.split_whitespace()
        .filter_map(|x| parse_number(x))
        .collect());
    store
}
fn parse_number(x: &str) -> Option<u32> {
    match x {
        x if x.ends_with('k') => x.trim_end_matches('k')
                                    .parse::<f32>()
                                    .ok()
                                    .map(|x| (x * 1000.0) as u32),
        _ => x.parse::<u32>().ok(),
    }
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_and_save_on_heap() {
        let new_str = String::from("5.5k 8.9k 32");
        let a_h = transform_and_save_on_heap(new_str);
        let result = take_value_ownership(a_h);
        assert_eq!(result, [5500, 8900, 32]);
    }
}
