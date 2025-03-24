/// # Chapter 2 - Expressions
///
/// Create a diverging function for addition of 2 numbers.
#[allow(unreachable_code)]
fn main() {
    let a = 10;
    let b = 20;

    diverging_add(a, b);

    // This line will never be executed
    println!("This line will never be executed");
}

// diverging_add 函数返回一个 `!` 类型(发散函数)，表示它永远不会返回。
fn diverging_add(a: i32, b: i32) -> ! {
    let result = a + b;
    println!("The result of {} + {} is: {}", a, b, result);
    std::process::exit(0);
}
