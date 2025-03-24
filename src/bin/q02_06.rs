/// # Chapter 2 - Expressions
///
/// Using a match statement to print `one` for input `1`, `two` for `2` and so on, and `NaN` on default.
fn main() {
    assert_eq!(handle_input(1), "one");
    assert_eq!(handle_input(2), "two");
    assert_eq!(handle_input(3), "NaN");
}

fn handle_input(input: i32) -> &'static str {
    match input {
        1 => "one",
        2 => "two",
        _ => "NaN",
    }
}
