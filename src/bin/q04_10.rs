use regex::Regex;

/// # Chapter 4 - Enum & Patterns
///
/// Create a `Regex` to extract dates from this string `It was on 2019-03-14, almost after a year from 2018-02-11` and
/// store in a Struct with fields of `day`, `month` and `year`.
/// `hint:` Use `Regex` crate
fn main() {
    let rgx = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let input = "It was on 2019-03-14, almost after a year from 2018-02-11";

    let mut matched_dates = Vec::new();

    for cap in rgx.captures_iter(input) {
        let date = Date {
            year: cap[1].parse().unwrap(),
            month: cap[2].parse().unwrap(),
            day: cap[3].parse().unwrap(),
        };
        matched_dates.push(date);
    }

    assert_eq!(matched_dates.len(), 2);

    assert_eq!(matched_dates[0].year, 2019);
    assert_eq!(matched_dates[0].month, 3);
    assert_eq!(matched_dates[0].day, 14);

    assert_eq!(matched_dates[1].year, 2018);
    assert_eq!(matched_dates[1].month, 2);
    assert_eq!(matched_dates[1].day, 11);
}

struct Date {
    day: i32,
    month: i32,
    year: i32,
}
