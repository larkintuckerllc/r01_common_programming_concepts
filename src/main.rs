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

    // NUMBERS
    let i = 1; // i32
    println!("The value of i is: {}", i); // 1
    let j = 1.1; // f64
    println!("The value of j is: {}", j); // 1.1

    // BOOLEAN
    let b = true; // bool
    println!("The value of b is: {}", b); // true

    // CHARACTER
    let c = 'a'; // char
    println!("The value of c is: {}", c); // a

    // TUPLE
    let tup = (0, 'a', 1.1); // (i32, char, f64)
    println!("The second value of tup is: {}", tup.1); // a
    let (t1, t2, t3) = tup;
    println!("he value of t1 is: {}", t1); // 0
    println!("he value of t2 is: {}", t2); // a
    println!("he value of t3 is: {}", t3); // 1.1

    // ARRAY
    let arr = [0, 1, 2]; // [i32, 3]
    println!("The second value of arr is: {}", arr[1]); // 1
    let [a1, a2, a3] = arr;
    println!("he value of a1 is: {}", a1); // 0
    println!("he value of a2 is: {}", a2); // 0
    println!("he value of a3 is: {}", a3); // 0

    // STRING

    // ENUM

    // ANY

    // VOID

    // NULL AND UNDEFINED

    // NEVER
    
    // OBJECT
}
