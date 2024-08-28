fn main() {
    // A) Assigning value of one variable to another variable
    let x = 5;
    let y = x;

    println!("Value of x: {}", x);
    println!("Value of y: {}", y);

    // B) Passing value to a function
    let z = add_numbers(x, y);
    println!("Sum of x and y: {}", z);

    // C) Returning value from a function
    let result = multiply_numbers(x, y);
    println!("Product of x and y: {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}