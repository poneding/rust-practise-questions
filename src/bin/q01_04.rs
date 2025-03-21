/// Chapter 1 - Basics
///
/// Take input(name) from the user of type `String` and print `Hello, <name>!`.
use std::io::stdin;
fn main() {
    println!("Enter your name: ");

    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("failed to read your name");

    println!("Hello, {}!", name.trim());
}
