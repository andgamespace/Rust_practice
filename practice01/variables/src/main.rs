use std::io;
// Rust is a statically typed language, which means that the type of a variable must be known at compile time.
// However, Rust has a powerful type inference system that can often infer the type of a variable based on its value.
// This means that you don't always have to explicitly specify the type of a variable.

// In Rust, variables are immutable by default. This means that once a variable is bound to a value, you cannot change that value.
// If you want to make a variable mutable, you need to use the mut keyword.

// This is a simple program that demonstrates the use of variables, data types, and basic arithmetic operations in Rust.
// It also demonstrates the use of shadowing, which allows you to reuse the same variable name in a different scope.
// Shadowing is a feature in Rust that allows you to declare a new variable with the same name as an existing variable.
// The new variable shadows the old variable, meaning that the old variable is no longer accessible in the new scope.
// This can be useful for creating new variables that are derived from existing variables, or for reusing variable names in different scopes.
// In Rust, you can use the let keyword to declare a variable. The syntax is as follows:
// let variable_name = value;
// You can also use the mut keyword to declare a mutable variable. The syntax is as follows:
// let mut variable_name = value;
// You can also use the const keyword to declare a constant. The syntax is as follows:
// const CONSTANT_NAME: TYPE = value;
// Constants are always immutable, and they must have a type annotation.
// Constants can be declared in any scope, including the global scope.
// Constants are always uppercase with underscores.
// In Rust, you can use the println! macro to print to the console. The syntax is as follows:
//This code panics if the index is out of bounds.

fn main() {
    let mut x = 3;
    println!("The value of x is {x}");
    x = 5;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
        // shadowing: the inner x shadows the outer x
        // the outer x is still accessible
        // but the inner x is used in this scope
        // the inner x is immutable
        // the inner x is a new variable
        // the inner x is not a reference to the outer x
        // the inner x is a new variable with the same name
    }
    let sum = 5 + 8;
    let t= true;
    let f: bool = false;
    let a = [1,2,3,4,5];
    let first = a[0];
    let second= a[1];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");
    let element = a[index];

    println!("The value of first is {first}");
    println!("The value of t is {t}");
    println!("The value of f is {f}");
    println!("the value of sum is {sum}");
    //addition
    let difference = 93.34 - 32.12;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("the value of difference is {difference}");
    //multiplication
    let product = 4 * 30;
    println!("the value of product is {product}");
    let quotient = 56.7 / 32.2;
    println!("the value of quotient is {quotient}");
    let truncated = -5 / 3; // results in -1
    println!("the value of truncated is {truncated}");
    let remainder = 43 % 5;
    println!("the value of remainder is {remainder}");
    println!(" The value of x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
    // rust uses snake_case for variable names
    // also constants are immutable by default, 
    // and they are defined using the const keyword
    // and they must have a type annotation
    // and they can be declared in any scope, including the global scope
    // they are uppercase with underscores
}
