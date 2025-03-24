/// # Chapter 7 - Closures & Iterators
///
/// Create a function that accepts a closure
fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    apply(&mut nums, |n: &mut i32| *n = *n * 3);

    for i in nums {
        println!("{}", i);
    }
}

fn apply<F>(nums: &mut Vec<i32>, mut f: F)
where
    F: FnMut(&mut i32),
{
    for n in nums.iter_mut() {
        f(n);
    }
}
