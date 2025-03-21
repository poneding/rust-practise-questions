/// # Chapter 11 - Concurrency
///
/// Find out the number of physical and logical cores in your CPU using rust. hint: try using `num_cpus` crate.
fn main() {
    // 使用 num_cpus::get() 获取逻辑 CPU 数量
    let logic_cpus = num_cpus::get();
    println!("Logical CPUs: {}", logic_cpus);

    // 使用 num_cpus::get_physical() 获取物理 CPU 数量
    let physical_cpus = num_cpus::get_physical();
    println!("Physical CPUs: {}", physical_cpus);
}
