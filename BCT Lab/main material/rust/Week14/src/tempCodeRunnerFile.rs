use rand::Rng;
use std::io;

fn main() {
    // Generate a secret number between 1 and 100
    let secret_number = rand::random::<u32>() % 20 + 1;

    println!("Guess the number!");

    while true {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("Congratulations! You guessed the secret number!");
            break;
        }
    }
}