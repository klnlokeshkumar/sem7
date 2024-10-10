fn main() {
    arr();
    tup();
}

fn arr() {
    println!("demonstration of arrays.");
    let a: [i32; 6] = [42, 57, 95, 21, 32, 85];
    println!("array elements: {:?}", a);
    println!("accessing 5th element:");
    println!("5th element = {}", a[4]);
}

fn tup() {
    println!("\ndemonstration of tuple");
    let b: (&str, i32, bool) = ("dileep", 20, true);
    println!("tuple elements: {:?}", b);
    println!("accessing 2nd element:");
    println!("2nd element = {}", b.1);
}