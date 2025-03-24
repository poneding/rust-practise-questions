use std::io::stdin;

/// # Chapter 2 - Expressions
///
/// Check whether the input number is odd or even and print `odd` or `even` respectively.
fn main() {
    loop {
        let mut input = String::new();
        println!("Enter a number to check if it's odd or even:");
        stdin()
            .read_line(&mut input)
            .expect("failed to read input from user");

        if let Ok(num) = input.trim().parse::<i32>() {
            let t = if num % 2 == 0 { "even" } else { "odd" };
            println!("{}", t);
        } else {
            println!("Please enter a valid number, got: {}", input);
        }
    }
}
