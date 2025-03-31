/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create a struct named `ComplexNumber` that has 2 variables `re` & `im` and overload `minus`
/// and `plus` operator to add and subtract complex number.
fn main() {
    let complex1 = ComplexNumber::new(1.0, 2.0);
    let complex2 = ComplexNumber::new(3.0, 4.0);

    let sum = complex1 + complex2;
    let difference = complex1 - complex2;

    assert_eq!(sum.re, 4.0);
    assert_eq!(sum.im, 6.0);

    assert_eq!(difference.re, -2.0);
    assert_eq!(difference.im, -2.0);
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

impl std::ops::Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: ComplexNumber) -> Self::Output {
        ComplexNumber::new(self.re + other.re, self.im + other.im)
    }
}

impl std::ops::Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, other: ComplexNumber) -> Self::Output {
        ComplexNumber::new(self.re - other.re, self.im - other.im)
    }
}
