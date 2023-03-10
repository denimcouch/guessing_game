use std::io;

fn main() {
    // Guessing Game
    // Step 1: Ask user for input
    println!("Guess the number!");
    
    println!("Please input your guess: ");
    
    let mut guess = String::new();
    
    // Step 2: Process the input and check that the input is in the expected form
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {guess}");
}
