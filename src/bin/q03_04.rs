use chrono::{Datelike, Duration, NaiveDate};

/// # Chapter 3 - Structs
///
/// Create a structure named Date having day, month and year as its elements. Store the current date in the structure. Now
/// add 45 days to the current date and display the final date.
fn main() {
    let curr_date = Date {
        day: 1,
        month: 12,
        year: 2020,
    };

    let final_date = add_days(curr_date, 45);
    assert_eq!(
        final_date,
        Date {
            day: 15,
            month: 1,
            year: 2021
        }
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Date {
    day: u32,
    month: u32,
    year: i32,
}

fn add_days(date: Date, days: i64) -> Date {
    let naive_date = NaiveDate::from_ymd_opt(date.year, date.month, date.day).unwrap();
    let final_date = naive_date + Duration::days(days);

    Date {
        day: final_date.day(),
        month: final_date.month(),
        year: final_date.year(),
    }
}
