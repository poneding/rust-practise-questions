/// # Chapter 5 - Traits & Generics
///
/// Implement custom `Iterator` trait for a struct named `GeometricSeries` that has
/// 3 fields `first_number`, `current_number` and `ratio` of type `i32`. The iterator should return the
/// next 11 numbers in geometric progression.
/// `hint:` Use `.take(11)` to get the next 11 in `for` loop.
fn main() {
    let geometric_series = GeometricSeries::new(2, 3);

    for number in geometric_series.take(11) {
        println!("{}", number);
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

#[cfg(test)]
mod test {
    #[test]
    fn test_wrapping_mul() {
        // wrapping_mul，避免相乘运算结果溢出，如果溢出，从最小值开始继续计算
        // 例如 u8 类型，250*2 运算会溢出导致 panic
        // 250.wrapping_mul(2) = 244
        let a: u8 = 250;
        let b = 2;
        assert_eq!(a.wrapping_mul(b), 244); // 500-256 
    }
}
