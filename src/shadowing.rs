fn main()
{
    // Shadowing
    // Shadowing is a feature in Rust that allows you to declare a new variable with the same name as a previous variable.
    // Allows to reuse variables without mutability
    // The new variable "shadows" the old variable, meaning that the old variable is no longer accessible after the new variable is declared.
    // This can be useful for reusing variable names in different scopes or for changing the type of a variable.

    let x = 5;
    println!("The value of x is: {}", x); //Output: 5

    let x = x + 1; // shadowing
    println!("The value of x is: {}", x); //Output: 6

    let x = x * 2; // shadowing
    println!("The value of x is: {}", x); //Output: 12
    // Shadowing can also be used to change the type of a variable
    let x = "Hello, world!"; // shadowing
    println!("The value of x is: {}", x); //Output: Hello, world!
    // The old variable is no longer accessible after the new variable is declared
    // println!("The value of x is: {}", x); // This will cause a compile-time error

    // Shadowing Vs Mutability
    /*
       - With mutability, the type of the variable must remain the same, and the variable can be changed in place.
       - With shadowing, the type of the variable can change, and the old variable is no longer accessible after the new variable is declared.
     */
}