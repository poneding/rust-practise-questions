use std::ops::Add;

/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Rust does not allow addition of `integer` and `float`. Overload `+` so that this is possible.
fn main() {
    let a = IntFloat::Int(5);
    let b = IntFloat::Float(3.5);
    let c = IntFloat::Int(2);
    let d = IntFloat::Float(4.5);

    assert_eq!(a + b, 8.5);
    assert_eq!(c + d, 6.5);
    assert_eq!(b + a, 8.5);
    assert_eq!(d + c, 6.5);
    assert_eq!(a + c, 7.0);
    assert_eq!(b + d, 8.0);
    assert_eq!(c + a, 7.0);
}

#[derive(Debug, Clone, Copy)]
enum IntFloat {
    Int(i32),
    Float(f32),
}

impl Add for IntFloat {
    type Output = f32;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (IntFloat::Int(i), IntFloat::Int(j)) => (i + j) as f32,
            (IntFloat::Float(i), IntFloat::Float(j)) => i + j,
            (IntFloat::Int(i), IntFloat::Float(j)) => (i as f32) + j,
            (IntFloat::Float(i), IntFloat::Int(j)) => i + (j as f32),
        }
    }
}
