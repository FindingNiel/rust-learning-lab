// use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    //mut
    let mut x = 1;
    println!("x: {x}");
    x = another_function(3);
    println!("x: {x}");

    let y = {
        let x= x * x;
        x + 1
    };

    println!("The value of y is: {y}");
    println!("You are also a {}", ten());

    //Shadowing?
    let y = square(y);
    println!("Square of y: {y}");
    println!("Largest number between x ({}) and y ({})? {}", x, y, larger(y, x))
}

fn another_function(x: i32) -> i32 {
    println!("Paramater: {x}");
    return 3;
}

fn ten() -> i32 {
    10
}

fn square(x: i32) -> i32 {
    x * x
}

fn larger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }

    // match a.cmp(&b) {
    //     Ordering::Less => b,
    //     Ordering::Greater => a,
    //     Ordering::Equal => a
    // }
}