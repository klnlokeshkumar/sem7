fn main() {
    let a:i32 = 0;
    let b:i32 = 1;
    println!("Bitwise AND: {}", a & b);
    println!("Bitwise OR: {}", a | b);
    println!("Bitwise XOR: {}", a ^ b);
    println!("Logical AND: {}", a > 0 && b > 0);
    println!("Logical OR: {}", a > 0 || b > 0);
    println!("Logical NOT: {}", !(a > 0));
}
