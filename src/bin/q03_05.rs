use std::default;

/// # Chapter 3 - Structs
///
/// Replicate constructor function with name `new()` returning type `Self`.
fn main() {
    let u = User::new("Pone Ding", "poneding@gmail.com");

    assert_eq!(u.username, "Pone Ding");
    assert_eq!(u.email, "poneding@gmail.com");
    assert_eq!(u.sign_in_count, 0);
    assert_eq!(u.active, true);
}

#[derive(Default)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(name: &str, email: &str) -> Self {
        Self {
            username: String::from(name),
            email: String::from(email),
            active: true,
            ..default::Default::default()
        }
    }
}
