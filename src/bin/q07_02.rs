/// # Chapter 7 - Closures & Iterators
///
/// Create an vec of numbers from `0` ... `100`, mutate the vec so that
/// it does not contain any number divisible by `3`. Use `retain` method of Vec.
fn main() {
    let mut nums = vec![0; 101];
    for i in 0..=100 {
        nums[i] = i;
    }

    nums.retain(|x| x % 3 != 0);

    for i in nums {
        println!("{}", i);
    }
}
