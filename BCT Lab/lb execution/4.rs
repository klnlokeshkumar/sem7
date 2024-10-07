use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let mut a: i32 = s.trim().parse().expect("Please type a number!");

    s.clear();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let mut b: i32 = s.trim().parse().expect("Please type a number!");
    println!("Before Swapping, a = {}, b = {}", a, b);
    a = a + b;
    b = a - b;
    a = a - b;
    println!("After Swapping, a = {}, b = {}", a, b);
}
