fn main() {
    let a: u8 = 0b1100;
    let b: u8 = 0b1010;

    println!("Bitwise AND: {:08b}", a & b);
    println!("Bitwise OR: {:08b}", a | b);
    println!("Bitwise XOR: {:08b}", a ^ b);
    println!("Bitwise NOT: {:08b}", !a);
    println!("Left shift: {:08b}", a << 1);
    println!("Right shift: {:08b}", a >> 1);

    let x = true;
    let y = false;

    println!("Logical AND: {}", x && y);
    println!("Logical OR: {}", x || y);
    println!("Logical NOT: {}", !x);
}
