/// # Chapter 2 - Expressions
///
/// WAP to iterate over 2 vectors at once.
/// `hint:` try using `.zip` method.
fn main() {
    let nums1 = vec![1, 2, 3, 4, 5, 6];
    let nums2 = vec![10, 20, 30, 40, 50];

    // .zip 方法可以将两个迭代器合并成一个迭代器
    // 值得注意的是：zip 方法会在最短的迭代器结束时停止迭代
    for (n1, n2) in nums1.iter().zip(nums2.iter()) {
        println!("{} {}", n1, n2);
    }
}
