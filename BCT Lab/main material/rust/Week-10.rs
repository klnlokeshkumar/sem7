use std::io;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Tuple components
    let mut int_input = String::new();
    let mut float_input = String::new();
    let mut string_input = String::new();
    let mut arr = [0; 3];
    let mut name_input = String::new();
    let mut age_input = String::new();

    // Input for integer
    println!("Enter an integer:");
    io::stdin().read_line(&mut int_input).expect("Failed to read line");
    let int_value: i32 = int_input.trim().parse().expect("Please enter a valid integer");

    // Input for float
    println!("Enter a float:");
    io::stdin().read_line(&mut float_input).expect("Failed to read line");
    let float_value: f64 = float_input.trim().parse().expect("Please enter a valid float");

    // Input for array
    println!("Enter 3 integers for the array:");
    for i in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr[i] = input.trim().parse().expect("Please enter a valid number");
    }

    // Input for string
    println!("Enter a string:");
    io::stdin().read_line(&mut string_input).expect("Failed to read line");
    let string_value = string_input.trim().to_string();

    // Input for struct
    println!("Enter a name for the struct:");
    io::stdin().read_line(&mut name_input).expect("Failed to read line");
    let name = name_input.trim().to_string();

    println!("Enter an age for the struct:");
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid age");

    let person = Person { name, age };

    // Create the tuple
    let tuple = (int_value, float_value, arr, string_value, person);

    // Print the tuple
    println!("Tuple: {:?}", tuple);
    println!("Struct inside tuple: Name = {}, Age = {}", tuple.4.name, tuple.4.age);
}
