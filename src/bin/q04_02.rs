/// # Chapter 4 - Enum & Patterns
///
/// Write an enum to store information of whether a person is a child or adult based on his/her age.
fn main() {
    let _child = Person::Child;

    let adult = Person::Adult(25);
    if let Person::Adult(age) = adult {
        println!("Adult with age: {}", age);
    }
}

enum Person {
    Child,
    Adult(u8), // Adult with age
}
