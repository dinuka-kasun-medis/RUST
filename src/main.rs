/**
 * @file main.rs    
 */
fn main() {
    println!("Hello, world!");

    let x = 5; //RUST variables are immutable by default, to ensure safety and concurrency
    // x = 6; // This will cause a compile-time error

    let mut y = 10; // mutable variable
    y = 20; // This is allowed because y is mutable

    // Data types
    /*
    RUST data types can be categorized into two main categories:
    1. Scalar types: represent a single value
        - Integer types: 
            - signed (Can represent both positive and negative numbers) -> i8, i16, i32, i64, i128, isize  (i32 is the default)
            - unsigned (Can only represent positive numbers) -> u8, u16, u32, u64, u128, usize
        - Floating-point types: f32, f64 (f64 is the default)
        - Boolean type: bool
        - Character type: char
            - represents a single Unicode character
            - represented by single quotes, e.g., 'a', '1', '!', '中'
            - can be any Unicode character, not just ASCII
            - even emojis are valid characters, example: '😊'
    2. Compound types: can group multiple values into one type
        - Tuple: a fixed-size collection of values of different types
        - Array: a fixed-size collection of values of the same type
     */

    // Example of scalar types
    let a: i32 = 10; // signed integer
    let decimal = 98_222; // decimal integer
    let hex = 0xff; // hexadecimal integer
    let octal = 0o77; // octal integer
    let binary = 0b1111_0000; // binary integer
    let byte = b'A'; // byte (u8) -> represents a single byte

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true; // boolean
    let f: bool = false; // boolean

    let c= 'a'; // character
    let e: char = 'A'; // character
    let f: char = '😊'; // Unicode character

    // Example of compound types
    let tup: (i32, f64, char) = (500, 6.4, 'y'); // tuple
    let (x, y, z) = tup; // destructuring tuple
    println!("The value of x is: {}", x);

    //accessing tuple elements
    let five_hundred = tup.0; // accessing tuple elements by index
    let six_point_four = tup.1;
    let y = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of y is: {}", y);


    let arr = [1, 2, 3, 4, 5]; // array
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // array
    let first = arr[0]; // accessing array elements by index
    let second = arr[1];
    let third = arr[2];
    let fourth = arr[3];
    let fifth = arr[4];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    println!("The value of third is: {}", third);
    println!("The value of fourth is: {}", fourth);
    println!("The value of fifth is: {}", fifth);

    // Constants
    /*   
        - Constants are immutable and must have a type annotation
        - Constants can be declared in any scope, including the global scope
        - Constants are not limited to a specific data type
        - Constants can be declared using the const keyword
        - Constants are always immutable
        - Constants can be declared using the static keyword
        - Can not use mut keyword 
    */
    const MAX_POINTS: u32 = 100_000; // constant
     
    
    
}

// This is a comment

/* 
This is a multi-line comment
*/
