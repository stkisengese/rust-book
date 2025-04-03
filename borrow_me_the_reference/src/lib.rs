// Create the following functions:

//     delete_and_backspace: which receives a borrowed string, and processes it.
//   - represents the backspace key and + represents the delete key, so that "helll-o" 
//      and "he+lllo" are both converted to "hello". The - and + characters should be removed from the string.

//     do_operations: which borrows a vector of string literals representing simple addition
//      and subtraction equations. The function should replace the operation with the result.

pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut delete_count = 0;
    let mut backspace_count = 0;

    for c in s.chars() {
        match c {
            '+' => delete_count += 1,
            '-' => backspace_count += 1,
            _ => {
                if delete_count > 0 {
                    delete_count -= 1;
                } else if backspace_count > 0 {
                    backspace_count -= 1;
                    result.pop();
                } else {
                    result.push(c);
                }
            }
        }
    }

    *s = result;
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

