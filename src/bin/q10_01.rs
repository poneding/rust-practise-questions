/// # Chapter 10 - Macros
///
/// Implement a macro named `addition` to add any amount of numbers. `Eg`: `addition!(5, 6, 57 ,56, 1)`
/// should return `125` and `addition!(4, 9)` should return `11`.
fn main() {
    let sum = addition!(1, 2, 3, 4, 5);
    assert_eq!(sum, 15);
}

// 参考 vec! 宏的实现，我们可以使用类似的方式来实现 addition! 宏。
#[macro_export]
macro_rules! addition {
    () => {
        0
    };
    ($elem:expr) => {
        $elem
    };
    ($($x:expr),+ $(,)?) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )+
            sum
        }
    };
}
