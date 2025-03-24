/// # Chapter 4 - Enum & Patterns
///
/// Use pattern matching to associate a enum of `Grade` type with a student based on his/her marks.
fn main() {
    let score = 85;
    assert_eq!(assign_grade(score), Grade::B);

    let score = 95;
    assert_eq!(assign_grade(score), Grade::A);

    let score = 75;
    assert_eq!(assign_grade(score), Grade::C);

    let score = 65;
    assert_eq!(assign_grade(score), Grade::D);

    let score = 55;
    assert_eq!(assign_grade(score), Grade::F);
}

#[derive(Debug, PartialEq)]
enum Grade {
    A,
    B,
    C,
    D,
    F,
}

fn assign_grade(score: u8) -> Grade {
    match score {
        90..=100 => Grade::A,
        80..=89 => Grade::B,
        70..=79 => Grade::C,
        60..=69 => Grade::D,
        _ => Grade::F,
    }
}
