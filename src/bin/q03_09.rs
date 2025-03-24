use std::cell::Cell;

/// # Chapter 3 - Structs
///
/// Create an immutable struct with a mutable member.
/// `hint:` Use `Cell`, property known as interior mutability.
fn main() {
    let immutable = ImmutableStruct {
        immutable_field: 5,
        mutable_field: Cell::new(5),
    };

    immutable.mutable_field.set(10);

    assert_eq!(immutable.mutable_field.get(), 10);
}

#[allow(dead_code)]
struct ImmutableStruct {
    immutable_field: i32,
    // Cell 是一种用于在不可变类型内部提供可变性的类型
    mutable_field: Cell<i32>,
}
