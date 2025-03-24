/// # Chapter 3 - Structs
///
/// Create a struct with `Copy, Clone, Debug, PartialEq` traits
/// `hint:` Use `#[derive]`
fn main() {
    let u = User {
        id: 1,
        name: "Pone Ding",
        age: 32,
    };

    assert_eq!(
        u,
        User {
            id: 1,
            name: "Pone Ding",
            age: 32
        }
    );
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct User<'a> {
    id: i32,
    name: &'a str,
    age: u8,
}
