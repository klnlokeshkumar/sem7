fn main() {

    let mut count = 0;
    while count < 3 {
        println!("While loop count: {}", count);
        count += 1;
    }

    for i in 1..=3 {
        println!("For loop count: {}", i);
    }

    let mut i = 0;
    loop {
        println!("Loop count: {}", i);
        i += 1;
        if i >= 3 {
            break;
        }
    }

    let mut optional_number = Some(5);
    while let Some(number) = optional_number {
        println!("Conditional loop: {}", number);
        optional_number = None;
    }
}
