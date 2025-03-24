/// # Chapter 2 - Expressions
///
/// Write a program to find prime numbers upto a number `N`
fn main() {
    assert_eq!(find_primes_upto_n(10), vec![2, 3, 5, 7]);
    assert_eq!(find_primes_upto_n(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
}

/// find_primes_upto_n 找出 n 以内的所有质数
fn find_primes_upto_n(n: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    for i in 2..=n {
        if is_prime(i) {
            primes.push(i);
        }
    }
    primes
}

/// is_prime 判断一个数是否是质数
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }

    // 平方根
    let sqrt_n = (n as f64).sqrt() as i32 + 1;
    // 从 2 到 sqrt_n 遍历，如果 n 能被整除，则 n 不是质数
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
