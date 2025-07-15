fn main() {
    println!("Hello, world! Welcome to Rust Control Flow!");

    /* println!();
    _showcase_if_condition(); */

    println!();
    _showcase_loop_control();
}

fn _showcase_loop_control() {
    println!("B. Loop control examples:");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter={}", counter);
        if counter == 10 {
            println!("breaking the loop");
            break counter * 1;
        }
    };

    println!("Result of loop: {}", result);

    let mut count = 0;

    'counting_up: loop {
        println!("Counting up: {}", count);
        count += 1;
        if count == 10 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("RISE!");

    let mut j = 0;
    while j < 5{
        println!("j = {}", j);
        j += 1;
    }

    let a=[1,2,3,4,5];
    for i in a {
        println!("i = {}", i);
    }

    for count in (1..=5).rev() {
        println!("count = {}", count);
    }

}

fn _showcase_if_condition() {
    println!("A. If condition examples:");

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

    let condition = false;
    let y = if condition { 5 } else { 10 };
    println!("y = {}", y);
}
