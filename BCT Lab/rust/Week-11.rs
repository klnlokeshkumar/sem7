fn main() {
    // Example of a loop
    let mut count = 0;
    loop {
        println!("Count: {}", count);
        count += 1;
        if count >= 5 {
            break;
        }
    }

    // Example of a while loop
    let mut num = 0;
    while num < 3 {
        println!("Number: {}", num);
        num += 1;
    }

    // Example of a for loop
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // Example of a conditional loop
    let mut i = 0;
    loop {
        if i % 2 == 0 {
            println!("Even number: {}", i);
        } else {
            println!("Odd number: {}", i);
        }
        i += 1;
        if i >= 5 {
            break;
        }
    }
}