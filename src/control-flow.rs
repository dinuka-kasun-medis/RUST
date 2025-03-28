/*
RUST conditional statements:
1. if statement: 
    - used to execute a block of code based on a condition
    - can be used as an expression (returns a value)
    - can have multiple branches using else if and else
2. match statement:
    - used to match a value against a pattern
    - can be used to destructure enums, tuples, and other types
    - can have multiple arms (branches) with different patterns
3. if let statement:
    - used to match a value against a pattern and execute a block of code if the pattern matches
    - can be used to destructure enums, tuples, and other types
    - can be used to handle Option and Result types
4. while let statement:
    - used to match a value against a pattern and execute a block of code while the pattern matches
    - can be used to destructure enums, tuples, and other types
    - can be used to handle Option and Result types
5. for loop:
    - used to iterate over a range of values or a collection
    - can be used to destructure enums, tuples, and other types
    - can be used to handle Option and Result types
6. loop statement:
    - used to create an infinite loop
    - can be used to break out of the loop using the break statement
    - can be used to return a value from the loop using the return statement
7. break statement:
    - used to break out of a loop
    - can be used to return a value from the loop using the return statement
8. continue statement:
    - used to skip the current iteration of a loop and continue to the next iteration
    - can be used to return a value from the loop using the return statement
9. return statement:
    - used to return a value from a function
    - can be used to return a value from a loop using the return statement
*/

fn main()
{
    let number = 6;
    // if statement
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }


    // match statement
    match number {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        4 => println!("The number is four"),
        _ => println!("The number is greater than four"),// _ is a wildcard pattern that matches any value
    }

    // if let statement
    let some_option = Some(5);
    if let Some(value) = some_option {
        println!("The value is: {}", value);
    } else {
        println!("The value is None");
    }

    // while let statement
    let mut some_option = Some(5);
    while let Some(value) = some_option {
        println!("The value is: {}", value);
        some_option = None; // to break the loop
    }

    // for loop
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("The value is: {}", i);
    }

    for i in arr {
        println!("The value is: {}", i);
    }

    // for loop with range
    for i in 1..6 { // 1..6 is a range that includes 1 and excludes 6
        println!("The value is: {}", i);
    }

    // loop statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // break out of the loop
        }
        println!("Count is: {}", count);
    }
    // labelled loop
    // labelled loop helps to break out of nested loops, and it is used to specify which loop to break out of
    /* 
        'outer: loop {
            'inner: loop {
                println!("In inner loop");
                break 'outer; // break out of the outer loop
            }
            println!("In outer loop");
        }
    */

    // break statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // break out of the loop
        }
        println!("Count is: {}", count);
    }

    // continue statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            continue; // skip the current iteration
        }
        println!("Count is: {}", count);
        if count == 10 {
            break; // break out of the loop
        }
    }

    // return statement
    /*
        let result = return_value(5);
        println!("The result is: {}", result);
    */


}