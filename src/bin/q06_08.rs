use std::fmt::Display;

/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Create a struct named `Table` that has a generic field `legs_info` of type `T`. Create a function (not a method)
/// named `show` that accepts function parameters `&Table<Display>` and displays `legs_info`. Create 2 variables one
/// that contains `T` of type `String: "Work in progress..."` and `usize: 4`.
/// `hint:` use `?Sized`.
fn main() {
    let table1 = Table {
        legs_info: "Work in progress...".to_string(),
    };
    let table2 = Table { legs_info: 4 };

    table1.show();
    table2.show();
}

// ?Sized 是一种特殊的 trait 边界，用于放宽类型 T 必须
// 具有已知大小的限制。默认情况下，泛型类型参数 T 必须在编译
// 时具有已知的大小，以便编译器可以为其分配内存。
struct Table<T: ?Sized> {
    legs_info: T,
}

impl<T: Display> Table<T> {
    fn show(&self) {
        println!("{}", self.legs_info);
    }
}
