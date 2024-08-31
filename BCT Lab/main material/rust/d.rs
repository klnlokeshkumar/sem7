use std::io;

fn main() {
    // Prompt the user for the first number
    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: f64 = first_number.trim().parse().expect("Invalid input");

    // Prompt the user for the second number
    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: f64 = second_number.trim().parse().expect("Invalid input");

    // Perform arithmetic operations
    let sum = first_number + second_number;
    let difference = first_number - second_number;
    let product = first_number * second_number;
    let quotient = first_number / second_number;

    // Display the results
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}