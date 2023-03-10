use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Guessing Game
    // Step 1: Ask user for input
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                                  .gen_range(1..=100); // Generate a random number between 1 and 100
    
    loop {
        println!("Please input your guess: ");
        
        let mut guess = String::new();
        
        // Step 2: Process the input and check that the input is in the expected form
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number.");
    
        println!("You guessed: {guess}");
    
        // Step 3: Compare the secret number and user input values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
