use easy_traits::*;

fn main() {
    let mut str_aux = StringValue {
        value: String::from(""),
    };
    str_aux.append_number(-1.0);
    println!("After append: {}", str_aux.value);


    

    println!("Before append: {}", str_aux.value);

    str_aux.append_str(String::from(" there!"));
    println!("After append: {}", str_aux.value);



    str_aux.remove_punctuation_marks();
    println!("After removing punctuation: {}", str_aux.value);
}
