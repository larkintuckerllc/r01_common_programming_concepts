fn another_function(x: i32) -> i32 {
    return x + 1;
}

fn main() {
    // IMMUTABLE
    let x = 5;
    println!("The value of x is: {}", x); // 5
    // x = 6; // CANNOT ASSIGN TWICE TO IMMUTABLE VARIABLE 
    // println!("The value of x is: {}", x);

    // MUTABLE
    let mut y = 5;
    println!("The value of y is: {}", y); // 5
    y = 6;
    println!("The value of y is: {}", y); // 6

    // RUN-TIME ASSIGNMENT
    let z = another_function(5);
    println!("The value of z is: {}", z); // 6
    let mut zz = another_function(5);
    zz = zz + 1;
    println!("The value of zz is: {}", zz); // 7

    // CONSTANT
    const MAX_POINTS: i32 = 100000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS); // 100000
    const ANOTHER_CONSTANT: i32 = 100000 + 1; // COMPILE-TIME ASSIGNMENT
    println!("The value of ANOTHER_CONSTANT is: {}", ANOTHER_CONSTANT); // 100001 
    // const YET_ANOTHER_CONSTANT: i32 = another_function(1); // CANNOT RUN-TIME ASSIGNMENT
    // println!("The value of YET_ANOTHER_CONSTANT is: {}", YET_ANOTHER_CONSTANT);

    // SHADOWING
    let a = 5;
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a); // 12
}
