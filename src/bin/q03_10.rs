use std::cell::RefCell;

/// # Chapter 3 - Structs
///
/// Try making the above program with `RefCell` so that the struct stores details about a bank(balance, customer count,
/// location, etc) with only customer count being mutable.
fn main() {
    // Cell 类型只适合用于 Copy 类型，因为它不提供所有权转移
    // RefCell 类型适用于引用类型，它会在运行时检查借用规则（而不是在编译时）
    let immutable = ImmutableStruct {
        immutable_field: 5,
        mutable_field: RefCell::new(5),
    };

    immutable.mutable_field.replace(10);

    assert_eq!(*immutable.mutable_field.borrow(), 10);
}

#[allow(dead_code)]
struct ImmutableStruct {
    immutable_field: i32,
    mutable_field: RefCell<i32>,
}
