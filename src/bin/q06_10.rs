/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create a function that accepts either `ComplexNumber` or `(isize, isize)` to return the `mod` of ComplexNumber.
fn main() {
    let c = ComplexNumber::new(3.0, 4.0);
    assert_eq!(calc_complex_mod(c), 5.0);

    let t = (3, 4);
    assert_eq!(calc_complex_mod(t), 5.0);
}

fn calc_complex_mod(c: impl Into<ComplexNumber>) -> f64 {
    let complex_number: ComplexNumber = c.into();
    complex_number.calc_mod()
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

impl ComplexNumber {
    fn calc_mod(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
}

/// 为 tuple (isize, isize) 实现 Into<ComplexNumber> trait
/// 以便可以将其转换为 `ComplexNumber`
/// `(isize, isize)` 可以作为 `impl Into<ComplexNumber>` 类型的参数传递
impl From<(isize, isize)> for ComplexNumber {
    fn from(tuple: (isize, isize)) -> Self {
        ComplexNumber::new(tuple.0 as f64, tuple.1 as f64)
    }
}
