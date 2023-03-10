use std::io;
use rand::Rng;

fn main() {
    // Guessing Game
    // Step 1: Ask user for input
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                                  .gen_range(1..=100); // Generate a random number between 1 and 100
    
    // println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");
    
    let mut guess = String::new();
    
    // Step 2: Process the input and check that the input is in the expected form
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {guess}");

}
