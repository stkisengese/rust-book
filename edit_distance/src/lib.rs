// Create a function named edit_distance, which calculates the minimum 
// number of changes (insertions, deletions and/or substitutions) which
// are needed to transform the source string to the target string.

pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.is_empty() { return target.len(); }
    if target.is_empty() { return source.len(); }

    let (s_head, s_tail) = source.split_at(1);
    let (t_head, t_tail) = target.split_at(1);

    if s_head == t_head {
        edit_distance(s_tail, t_tail)
    } else {
        1 + edit_distance(source, t_tail) // insert t_head into source
            .min(edit_distance(s_tail, target)) // delete s_head from source
            .min(edit_distance(s_tail, t_tail)) // substitute s_head with t_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        let result = edit_distance("alignment", "assignment");
        assert_eq!(result, 2);
    }
    #[test]
    fn test_edit_distance2() {
        let result = edit_distance("sitting", "kitten");
        assert_eq!(result, 3);
    }
}
