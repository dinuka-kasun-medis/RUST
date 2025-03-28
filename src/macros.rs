//Macros
    /* 
     - Macros are a way to write code that writes other code (metaprogramming).
     - Macros are usable to reduce boilerplate code and to create domain-specific languages (DSLs).

     - Macros come in two flavors:
        1. Declarative macros (macro_rules!): 
            - These are the most common type of macros in Rust.
            - They allow you to define a pattern and specify how to transform that pattern into code.
            - They are defined using the macro_rules! keyword.
        2. Procedural macros: 
            - These are more advanced and allow you to write custom syntax extensions.
            - They are defined as functions that take a TokenStream as input and produce a TokenStream as output.
            - They are used for more complex code generation tasks.
    */

// Example of a declarative macro
// The macro_rules! keyword is used to define a macro
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

// macro arguments, expr is a macro argument that can be any valid Rust expression
macro_rules! say_hello_with_name {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

// Real usage of macros, for logging
macro_rules! log {
    ($level:expr, $msg:expr) => {
        println!("[{}] {}", $level, $msg);
    };
}

//println! is a macro that prints to the console
// The println! macro is a built-in macro in Rust that prints to the console

fn main()
{
    say_hello!(); // This will expand to println!("Hello, world!");
    say_hello_with_name!("Alice"); // This will expand to println!("Hello, Alice!");
    log!("INFO", "This is an info message"); // This will expand to println!("[INFO] This is an info message");
}