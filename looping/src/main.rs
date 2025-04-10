use std::io;

fn main(){
    let riddle = "I am the beginning of the end, and the end of time and space. Iam essential to creation, and I sorround every place. What am I?";

    let correct_answer = "the letter e";
    let mut attempts = 0;

    loop {
        // print the riddle
        println!("{}", riddle);

        // get user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Remove trailing newline and convert to lowercase for comparison
        guess = guess.trim().to_lowercase();
        attempts += 1;

        // Check if the answer is correct
        if guess == correct_answer {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}
