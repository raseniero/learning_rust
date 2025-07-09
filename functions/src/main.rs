/**
 * This is a simple Rust program that demonstrates the use of functions.
 */
fn main() {
    println!("Functions");

    // Call another_function()
    another_function(1, 'a');

    // Call print_label_measurement() {
    print_label_measurement(2, 'b');

    // let x = let y= 10;
    let x = {
        let y = 10;
        y + 5
    };
    println!("Value of x: {}", x);

    // Call five()
    let x = five();
    println!("Value of x: {}", x);

    // Call plus_one()
    let x = plus_one(5);
    println!("Value of x after plus_one: {}", x);
}

/** Function five to return the value 5
 * Function plus_one to add 1 to the input value
 * Function another_function to print a message with two parameters
 * Function print_label_measurement to print a measurement label and value
 */
fn five() -> i32 {
    5
}

/**
 * Plus one function to add 1 to the input value
 */
fn plus_one(x: i32) -> i32 {
    x + 1
}

/**
 * Another function to print a message with two parameters
 */
fn another_function(x: i32, char: char) {
    println!("Another function! Value of x: {x}, char: {char}");
}

/**
 * Function to print a measurement label and value
 */
fn print_label_measurement(x: i32, measurement: char) {
    println!("Measurement: {measurement}, Value: {x}");
}
