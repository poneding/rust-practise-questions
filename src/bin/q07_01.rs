/// # Chapter 7 - Closures & Iterators
///
/// Create a closer `add_one` to add 1 to an integer.
fn main() {
    let add_one = |x: i32| x + 1;
    let x = 5;
    let result = add_one(x);
    let result = add_one(result);
    assert_eq!(result, 7);
}
