/// # Chapter 5 - Traits & Generics
///
/// Implement a generic function name `sum` with additional parameter of `index: usize`
/// that can take either `GeometricSeries` or `FibonacciSeries` and returns the sum upto the given index.
/// `hint:` use `T: Iterator<Item = i32>` where `T` is generic
fn main() {
    let f = FibonacciSeries::new();
    assert_eq!(sum(f, 5), 12);

    let g = GeometricSeries::new(2, 3);
    assert_eq!(sum(g, 5), 242);
}

fn sum<T: Iterator<Item = i32>>(t: T, index: usize) -> T::Item {
    let mut result = 0;
    for v in t.take(index) {
        result += v
    }
    result
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

/// GeometricSeries 等比数列，例如：2，6，18，等比数列的公比为 3
#[allow(dead_code)]
struct GeometricSeries {
    first_number: i32,
    current_number: i32,
    ratio: i32,
}

impl GeometricSeries {
    fn new(first_number: i32, ratio: i32) -> Self {
        Self {
            first_number,
            current_number: first_number,
            ratio,
        }
    }
}

impl Iterator for GeometricSeries {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_number;
        self.current_number = self.current_number.wrapping_mul(self.ratio);
        Some(result)
    }
}
