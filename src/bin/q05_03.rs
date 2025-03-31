/// # Chapter 5 - Traits & Generics
///
/// Implement custom `Drop` trait for a struct name `Student` that contains your `name`, `age` and `roll number`. It
/// should return `Roll number <roll number> has name <name> with age <age> and is a <junior/senior>`. Being Junior or
/// Senior depends on age (18 or above).
fn main() {
    let _student = Student {
        name: "John".to_string(),
        age: 20,
        roll_number: 123456,
    };
}

struct Student {
    name: String,
    age: u8,
    roll_number: u32,
}

impl Drop for Student {
    fn drop(&mut self) {
        let status = if self.age >= 18 { "Senior" } else { "Junior" };
        println!(
            "Roll number {} has name {} with age {} and is a {}",
            self.roll_number, self.name, self.age, status
        );
    }
}
