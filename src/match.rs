/*
RUST match statement:
    - used to match a value against a pattern
    - can be used to destructure enums, tuples, and other types
    - can have multiple arms (branches) with different patterns
    - can be used to handle Option and Result types
    - can be used to match against ranges of values
    - can be used to match against multiple values
    - can be used to match against complex patterns
*/

fn main()
{
    let number = 10;
    // match statement
    match number {
        1 => {
            println!("The number is one");
            // You can also use a block of code here
            let x = 10;
            println!("x is: {}", x);
        },
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        4 => println!("The number is four"),
        5 | 6 | 7  => println!("The number is five, six or seven"), // OR operator
        8..10 => println!("The number is between eight and ten"), // range operator include 8 and not include 10
        10..= 20 => println!("The number is between ten and twenty"), // range operator include 10 and 20
        _ => println!("The number is greater than twenty"),// _ is a wildcard pattern that matches any value
    }


    //Destructuring with match
    let point = (3, 4);
    match point {
        (0, 0) => println!("The point is at the origin"),
        (x, 0) => println!("The point is on the x-axis at {}", x),
        (0, y) => println!("The point is on the y-axis at {}", y),
        (x, y) => println!("The point is at ({}, {})", x, y),
        // Can not use the _ wildcard pattern here because it will match all values in above scenario
    }

    let pair = (2, -2);
    match pair {
        (x, y) if x==y => println!("The numbers are equal"),
        (x, y) if x+y == 0 => println!("The numbers are opposites"),
        _ => println!("The numbers are different"), // wildcard pattern
    }

    let ignoring_pair = (2, -2);
    match ignoring_pair {
        (x, _) => println!("The first number is: {}", x),// Ignoring the values in pattern
    }

    
}