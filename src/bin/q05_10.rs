use std::fmt::Display;

/// # Chapter 5 - Traits & Generics
///
/// Implement a trait named `Hello` with a default function `say(&self)` that prints `Hello, <self>!`
/// , Implement this for `str` and `string` without providing any definition of `Hello` (simply `impl Hello for str {}`)
/// call `say` on str `World`.
fn main() {
    let s: &str = "World";
    s.say();

    let s: String = "World".to_string();
    s.say();
}

trait Hello: Display {
    fn say(&self) {
        println!("Hello, {}!", self);
    }
}

impl Hello for str {}

impl Hello for String {}
