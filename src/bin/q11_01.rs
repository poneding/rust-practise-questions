use std::{thread, time::Duration};
/// # Chapter 11 - Concurrency
///
/// Spawn 2 threads, one that continuously says `Hello` and the other that say `World`.
fn main() {
    let print_hello_handle = thread::spawn(|| {
        loop {
            println!("Hello");
            thread::sleep(Duration::from_secs(1));
        }
    });
    print_hello_handle.join().unwrap(); // 等待线程结束

    let print_world_handle = thread::spawn(|| {
        loop {
            println!("World");
            thread::sleep(Duration::from_secs(1));
        }
    });
    print_world_handle.join().unwrap(); // 等待线程结束
}
