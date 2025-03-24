use std::io::stdin;

/// # Chapter 2 - Expressions
///
/// Take a `integer` input from the user and print table for that integer using a loop.
fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("failed to read input from user");
    let num: i32 = input
        .trim()
        .parse()
        .expect("failed to parse input to integer");

    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
