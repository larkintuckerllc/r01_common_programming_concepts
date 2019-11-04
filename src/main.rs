fn another_function(x: u32) -> u32 {
    return x + 1;
}

fn main() {
    // IMMUTABLE
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // CANNOT ASSIGN TWICE TO IMMUTABLE VARIABLE 
    println!("The value of x is: {}", x);
    let xx = another_function(5);
    println!("The value of xx is: {}", xx);

    // MUTABLE
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // CONSTANT
    const MAX_POINTS: u32 = 100000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // const ANOTHER_CONSTANT: u32 = another_function(1); // LIMITED
    // println!("The value of ANOTHER_CONSTANT is: {}", ANOTHER_CONSTANT);

    // SHADOWING
    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);
}
