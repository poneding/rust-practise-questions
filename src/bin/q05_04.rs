use std::fmt::Debug;

/// # Chapter 5 - Traits & Generics
///
/// Implement a custom `Debug` message for the above Struct.
fn main() {
    let student = Student {
        name: "John".to_string(),
        age: 20,
        roll_number: 123456,
    };
    
    assert_eq!(
        format!("{:?}", student),
        "Roll number 123456 has name John with age 20 and is a Senior"
    );
}

struct Student {
    name: String,
    age: u8,
    roll_number: u32,
}

impl Debug for Student{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.age >= 18 { "Senior" } else { "Junior" };
        write!(
            f,
            "Roll number {} has name {} with age {} and is a {}",
            self.roll_number, self.name, self.age, status
        )
    }
}