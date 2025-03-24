/// # Chapter 2 - Expressions
///
/// Find and print factorial of a program using recursion.
fn main() {
    let n = 5;

    let result = factorial_recursive(n);
    println!("Factorial of {} is: {}", n, result);

    let result = factorial_iterative(n);
    println!("Factorial of {} is: {}", n, result);
}

/// factorial_recursive 使用递归的方式计算阶乘
fn factorial_recursive(n: i64) -> i64 {
    if n <= 1 {
        return 1;
    }

    return factorial_recursive(n - 1) * n;
}

/// factorial_iterative 使用迭代的方式计算阶乘
fn factorial_iterative(n: i64) -> i64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
