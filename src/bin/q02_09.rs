/// # Chapter 2 - Expressions
///
/// Write a function to swap 2 numbers.
fn main() {
    let mut a = 10;
    let mut b = 20;
    println!("Before swap: a: {}, b: {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap: a: {}, b: {}", a, b);
}

// fn swap(a: &mut i32, b: &mut i32) {
//     let temp = *a;
//     *a = *b;
//     *b = temp;
// }

fn swap(a: &mut i32, b: &mut i32) {
    (*a, *b) = (*b, *a);
}
