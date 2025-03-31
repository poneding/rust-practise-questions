/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Implement `From` trait for struct `ComplexNumber` given a tuple of type `(isize, isize)`. Form it using `from`
/// and `into` respectively.
fn main() {
    let complex1 = ComplexNumber::from((3, 4));
    assert_eq!(complex1.re, 3.0);
    assert_eq!(complex1.im, 4.0);

    let complex2: ComplexNumber = (5, 6).into();
    assert_eq!(complex2.re, 5.0);
    assert_eq!(complex2.im, 6.0);
}

#[derive(Debug, Clone, Copy)]
struct ComplexNumber {
    re: f64,
    im: f64,
}

impl From<(isize, isize)> for ComplexNumber {
    fn from(value: (isize, isize)) -> Self {
        ComplexNumber {
            re: value.0 as f64,
            im: value.1 as f64,
        }
    }
}
