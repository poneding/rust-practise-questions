/// # Chapter 5 - Traits & Generics
///
/// Implement the same as above for a `FibonacciSeries` struct.
fn main() {
    let mut f = FibonacciSeries::new();
    assert_eq!(f.next(), Some(1));
    assert_eq!(f.next(), Some(2));
    assert_eq!(f.next(), Some(3));
    assert_eq!(f.next(), Some(5));
}

/// FibonacciSeries 斐波那契数列，例如：1，2，3，5，8
#[derive(Debug)]
struct FibonacciSeries {
    prvious_number: i32,
    current_number: i32,
}

impl FibonacciSeries {
    fn new() -> Self {
        Self {
            prvious_number: 0,
            current_number: 1,
        }
    }
}

impl Iterator for FibonacciSeries {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.current_number, self.prvious_number) = (
            self.current_number.wrapping_add(self.prvious_number),
            self.current_number,
        );

        Some(self.current_number)
    }
}
