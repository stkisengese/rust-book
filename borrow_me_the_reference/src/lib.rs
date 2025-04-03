// Create the following functions:

//     delete_and_backspace: which receives a borrowed string, and processes it.
//   - represents the backspace key and + represents the delete key, so that "helll-o" 
//      and "he+lllo" are both converted to "hello". The - and + characters should be removed from the string.

//     do_operations: which borrows a vector of string literals representing simple addition
//      and subtraction equations. The function should replace the operation with the result.

pub fn delete_and_backspace(e: &mut String) {
    let mut result = String::new();
    for c in e.chars() {
        match c {
            '-' => result.pop(),
            '+' => result.truncate(result.len() - 1),
            _ => result.push(c),
        }
    }
    *e = result;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(pos) = s.find('+') {
            let left = s[..pos].parse::<i32>().unwrap();
            let right = s[pos+1..].parse::<i32>().unwrap();
            *s = (left + right).to_string();
        } else if let Some(pos) = s.find('-') {
            let left = s[..pos].parse::<i32>().unwrap();
            let right = s[pos+1..].parse::<i32>().unwrap();
            *s = (left - right).to_string();
        }
    }
}

