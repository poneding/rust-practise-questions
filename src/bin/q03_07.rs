use std::ops::Add;

/// # Chapter 3 - Structs
///
/// Create a generic struct for addition of numbers (they can be `integer` or `float`).
fn main() {
    let adder1 = Adder { v: 1 };
    let adder2 = Adder { v: 2 };

    assert_eq!(adder1.add(adder2), 3);
}

#[derive(Debug)]
struct Adder<T> {
    v: T,
}

impl<T: Add<Output = T>> Adder<T> {
    fn add(self, rhs: Self) -> T {
        self.v + rhs.v
    }
}
