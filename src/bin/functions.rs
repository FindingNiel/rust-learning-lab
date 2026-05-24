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
    println!("Square of y: {}", square(y));
    println!("y: {}", y);
    println!("You are also a {}", ten());
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