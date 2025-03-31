/// # Chapter 5 - Traits & Generics
///
/// Derive a debug trait to print info about your struct that contains `name`, `c1ass` and `roll`.
fn main() {
    let student = Student {
        name: "John Doe".to_string(),
        class: "X".to_string(),
        roll: 1,
    };

    println!("{:?}", student);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Student {
    name: String,
    class: String,
    roll: i32,
}
