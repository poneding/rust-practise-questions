/// # Chapter 3 - Structs
///
/// Write a program to store and print the roll no., name, age and marks of a student using structures.
fn main() {
    let mut students = vec![];

    students.push(Student {
        roll_no: 1,
        name: "John".to_string(),
        age: 20,
        marks: "A".to_string(),
    });
    students.push(Student {
        roll_no: 2,
        name: "Jane".to_string(),
        age: 21,
        marks: "B".to_string(),
    });

    for student in students {
        println!(
            "Roll No: {}, Name: {}, Age: {}, Marks: {}",
            student.roll_no, student.name, student.age, student.marks
        );
    }
}

struct Student {
    roll_no: i32,
    name: String,
    age: i32,
    marks: String,
}
