use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 原子类型适合简单计数、状态标志，不需要 Mutex。
    let counter = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let stop = Arc::clone(&stop);
        handles.push(thread::spawn(move || {
            while !stop.load(Ordering::Relaxed) {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    thread::sleep(Duration::from_millis(50));
    stop.store(true, Ordering::Relaxed);

    for handle in handles {
        handle.join().expect("worker should finish");
    }

    println!("counter = {}", counter.load(Ordering::Relaxed));

    // Ordering::Relaxed 只保证原子性，不建立跨线程的先后可见关系。
    // 日常业务代码中，如果不确定内存顺序，优先用 Mutex/RwLock 或更高层并发结构。
}
