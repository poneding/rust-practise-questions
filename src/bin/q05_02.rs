/// # Chapter 5 - Traits & Generics
///
/// Create a generic function to get min of 2 values.
/// `hint:` You might need to use `Ord` trait bound.
fn main() {
    let (a,b) = (10,20);
    let min_n= min(a,b);
    println!("Min of {} and {} is {}", a, b, min_n);
    assert_eq!(min_n, a);
}

fn min<T: Ord>(a: T, b:T) -> T{
    if a < b {
        a
    } else {
        b
    }
}