fn main() {
    println!("Hello, world! to Rust's control flow!");

    let x = 10;
    println!("x = {}", x);

    if x > 10 {
        println!("x is > than 10");
    } else {
        println!("x is <= than 10");
    }

    if x < 5 {
        println!("x is < than 5");
    } else if x < 10 {
        println!("x is < than 10 but >= 5");
    } else {
        println!("x is >= than 10");
    }

    let condition=false;
    let y = if condition {5} else {10};
    println!("y = {}", y);
}
