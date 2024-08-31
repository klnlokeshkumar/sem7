fn main()
{
    let age:i32 = 20;
    let height:f32 = 5.9;
    let name:&str = "Lokesh";

    println!("Age:{},name:{},height:{}",age,name,height);
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", age, age, age);
}