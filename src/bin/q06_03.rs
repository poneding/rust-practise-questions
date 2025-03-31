use std::ops::Not;

/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Overload the `!` operator to conjugate the complex number `!ComplexNumber` and `==` and `!=` for comparison.
fn main() {
    let complex1 = ComplexNumber::new(1.0, 2.0);
    let complex2 = ComplexNumber::new(3.0, 4.0);

    let conjugate = !complex1;
    assert_eq!(conjugate.re, 1.0);
    assert_eq!(conjugate.im, -2.0);

    assert!(complex1 != complex2);
    assert!(complex1 == complex1);
}

#[derive(Debug, Clone, Copy)]
struct ComplexNumber {
    re: f64,
    im: f64,
}

impl ComplexNumber {
    fn new(re: f64, im: f64) -> Self {
        ComplexNumber { re, im }
    }
}

impl Not for ComplexNumber {
    type Output = ComplexNumber;

    fn not(self) -> Self::Output {
        ComplexNumber {
            re: self.re,
            im: -self.im,
        }
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}
