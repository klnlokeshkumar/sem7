fn main() {
    let mut x = 5;

    // Borrow x immutably
    let y = &x;
    println!("The value of y is: {}", y);

    // Borrow x mutably
    let z = &mut x;
    *z += 1;
    println!("The value of x after mutation is: {}", x);
}