use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let left = Arc::new(Mutex::new(0));
    let right = Arc::new(Mutex::new(0));

    // 死锁常见来源：不同线程以不同顺序获取多个锁。
    // 规避策略：所有地方都按相同顺序加锁。
    let worker_a = {
        let left = Arc::clone(&left);
        let right = Arc::clone(&right);
        thread::spawn(move || {
            increment_both("a", &left, &right);
        })
    };

    let worker_b = {
        let left = Arc::clone(&left);
        let right = Arc::clone(&right);
        thread::spawn(move || {
            increment_both("b", &left, &right);
        })
    };

    worker_a.join().expect("worker a should finish");
    worker_b.join().expect("worker b should finish");

    println!(
        "left = {}, right = {}",
        *left.lock().expect("left lock"),
        *right.lock().expect("right lock")
    );

    // try_lock 可以避免无限等待，适合需要降级或重试的场景。
    {
        if let Ok(guard) = left.try_lock() {
            println!("try_lock succeeded: {guard}");
        };
    }
}

fn increment_both(name: &str, first: &Mutex<i32>, second: &Mutex<i32>) {
    let mut first_guard = first.lock().expect("first lock");
    thread::sleep(Duration::from_millis(10));
    let mut second_guard = second.lock().expect("second lock");

    *first_guard += 1;
    *second_guard += 1;
    println!("worker {name} incremented both values");
}
