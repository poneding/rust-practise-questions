/// # Chapter 3 - Structs
///
/// Write a program to compare two dates entered by user. Make a structure named Date to store the elements day, month and
/// year to store the dates. If the dates are equal, display "Dates are equal" otherwise display "Dates are not equal".
fn main() {
    let date1 = Date {
        day: 1,
        month: 1,
        year: 2020,
    };
    let date2 = Date {
        day: 1,
        month: 1,
        year: 2020,
    };

    if date1 == date2 {
        println!("Dates are equal");
    } else {
        println!("Dates are not equal");
    }
}

struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.month == other.month && self.year == other.year
    }
}
