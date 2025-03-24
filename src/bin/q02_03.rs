/// # Chapter 2 - Expressions
///
/// Print the following pattern using loops.
/// ```
///      *
///     ***
///    *****
///   *******
///  *********
/// ```
fn main() {
    let mut i: usize = 0;
    loop {
        if i >= 5 {
            break;
        }

        let spaces = " ".repeat(5 - i - 1);
        let stars = "*".repeat(i * 2 + 1);
        println!("{}{}", spaces, stars);

        i += 1;
    }
}
