/// # Chapter 6 - Operator Overloading & Utility Traits
///
/// Implement custom `Drop` for a struct `Hero` that contains a field `name` of type string.
/// The `drop` should print "Oh no !!! Our hero {name} is defeated". Run the program with just
/// declaring a variable of type `Hero`.
fn main() {
    let h = Hero {
        name: "Superman".to_string(),
    };

    println!("Our hero is: {}", h.name);
}

struct Hero {
    name: String,
}

impl Drop for Hero {
    fn drop(&mut self) {
        println!("Oh no !!! Our hero {} is defeated", self.name);
    }
}
