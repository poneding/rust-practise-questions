/// # Chapter 2 - Expressions
///
/// Create a program to check for leap year.
/// `note:` 1900 is not a leap year.
fn main() {
    assert_eq!(check_leap_year(1900), false);
    assert_eq!(check_leap_year(2000), true);
    assert_eq!(check_leap_year(2004), true);
    assert_eq!(check_leap_year(2005), false);
}

/// check_leap_year 函数接收一个年份作为参数，返回一个布尔值，表示是否是闰年。
/// 闰年的定义如下：
/// 1. 能被4整除，但不能被100整除的年份是闰年（普通闰年）。
/// 2. 能被400整除的年份是闰年(世纪闰年)。
fn check_leap_year(year: i32) -> bool {
    match year {
        year if year % 4 == 0 && year % 100 != 0 => true,
        year if year % 400 == 0 => true,
        _ => false,
    }
}
