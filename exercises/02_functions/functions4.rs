/// This function returns the possible types as a result in Rust.
/// The possible return types in Rust can be specified using the `->` syntax after the function signature.
/// Here are some common return types in Rust:
/// - `()` - The unit type, representing no value.
/// - `bool` - A boolean value, either `true` or `false`.
/// - `i32`, `u32`, `i64`, `u64`, etc. - Signed and unsigned integer types of various sizes.
/// - `f32`, `f64` - Floating-point types of various sizes.
/// - `String` - A dynamically allocated string.
/// - `Vec<T>` - A dynamically sized vector of elements of type `T`.
/// - `Option<T>` - An optional value that can be either `Some(value)` or `None`.
/// - `Result<T, E>` - A result that can be either `Ok(value)` or `Err(error)`.
/// - Custom types defined by the user.


// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}


fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
