/// # Chapter 5 - Traits & Generics
///
/// Make a generic function to return the greater of the 2 given values (integer and floats).
fn main() {
    let a: i32 = 10;
    let b: i32 = 20;
    assert_eq!(greater(a, b), 20);

    let c: f32 = 10.0;
    let d: f32 = 20.0;
    assert_eq!(greater(c, d), 20.0);

    let e: f64 = 10.0;
    let f: f64 = 20.0;
    assert_eq!(greater(e, f), 20.0);
}

fn greater<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
