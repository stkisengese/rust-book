use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut trial_count = 0;

    loop {
        trial_count += 1;
        println!("{}", riddle);
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.trim().to_lowercase() == answer.to_lowercase() {
            println!("Number of trials: {}", trial_count);
            break;
        }
    }
}
