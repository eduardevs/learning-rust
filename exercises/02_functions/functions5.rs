// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
   return num * num;
/// This code requires adding a `return` statement or removing the semicolon (`;`) depending on the context.
/// 
/// In Rust, the `return` keyword is used to explicitly return a value from a function. If you want to return a value from a function, you need to use the `return` keyword followed by the value you want to return.
/// 
/// However, if you don't want to return a value from a function (e.g., for functions with a return type of `()`), you can omit the `return` keyword and simply end the expression with a semicolon (`;`). The semicolon indicates that the expression is a statement and not an expression that produces a value.
/// 
/// So, depending on the desired behavior of your code, you either need to add a `return` statement to explicitly return a value or remove the semicolon to indicate that the expression is a value-producing expression.
}
