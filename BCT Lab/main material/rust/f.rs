fn main()
{
    let mut x:i32 = 5;
    let mut y:i32 = 6;

    println!("before swapping x={},y = {}",x,y);

    x = x + y;
    y = x-y;
    x = x - y;
    println!("After swapping x={},y = {}",x,y);
}