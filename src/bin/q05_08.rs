use std::ops::Mul;

/// # Chapter 5 - Traits & Generics
///
/// Write a generic function name `Multiply` that multiplies all `usize`, `isize` and `fsize` type values.
fn main() {
    let v: Vec<usize> = vec![1, 2, 3, 4, 5];
    assert_eq!(multiply(&v), 120);

    let v: Vec<isize> = vec![-1, -2, -3, -4, -5];
    assert_eq!(multiply(&v), -120);

    let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(multiply(&v), 120.0);

    let v: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(multiply(&v), 120.0);
}

fn multiply<T>(values: &[T]) -> T
where
    T: Mul<T, Output = T> + Copy + Default,
{
    if values.is_empty() {
        return T::default();
    }

    let mut result = values[0];
    for &value in values.iter().skip(1) {
        result = result * value;
    }
    result
}
