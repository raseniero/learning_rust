fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("Value of x is {x}", x = x);

    x = 6;
    println!("Value of x is now {x}", x = x);

    const HOURS_IN_SECONDS: u32 = 60 * 60;
    println!(
        "There are {HOURS_IN_SECONDS} seconds in an hour",
        HOURS_IN_SECONDS = HOURS_IN_SECONDS
    );

    // Integers
    let u_y: u8 = u8::MAX;
    println!("u_y::MAX = {u_y}");

    let u_y: u8 = u8::MIN;
    println!("u_y::MIN={u_y}");

    let i_y: i8 = i8::MIN;
    println!("i_y::MIN={i_y}");

    let i_y: i8 = i8::MAX;
    println!("i_y::MAX={i_y}");

    let i_w: i16 = i16::MAX;
    println!("i_w::MAX={i_w}");

    let u_w: u16 = u16::MAX;
    println!("u_w::MAX={u_w}");

    // Floating points
    let f_x: f32 = f32::MAX;
    println!("f_x:MAX={f_x}");

    let f_x: f32 = f32::MIN;
    println!("f_x:MIN={f_x}");

    // Compound types
    // Tuple
    let tup = (500, 1.2, 1);
    let (x, y, z) = tup;
    println!("x={x}, y={y}, z={z}");
    println!("tup.0={0},tup.1={1},tup.2={2}", tup.0, tup.1, tup.2);
    println!("tup.0={}", tup.0);

    // Array
    let x = [1, 2, 3, 4, 5];
    let x = [2;5];
    println!("x[0]={}", x[0]);

    let a = [3; 5];
    println!("a[0]={}", a[0])
}
