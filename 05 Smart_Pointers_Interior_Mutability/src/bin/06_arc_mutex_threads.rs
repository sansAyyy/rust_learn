use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // Arc<T> 是线程安全的引用计数指针。
    // Mutex<T> 提供互斥访问，实现多线程内部可变性。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().expect("mutex should not be poisoned");
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("thread should finish");
    }

    println!("counter = {}", *counter.lock().expect("mutex should lock"));

    // RwLock<T> 允许多个读者，或一个写者。
    let config = Arc::new(RwLock::new(String::from("v1")));
    let reader_a = {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            let value = config.read().expect("rwlock should read");
            println!("reader_a sees {value}");
        })
    };
    let reader_b = {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            let value = config.read().expect("rwlock should read");
            println!("reader_b sees {value}");
        })
    };

    reader_a.join().expect("reader_a should finish");
    reader_b.join().expect("reader_b should finish");

    {
        let mut value = config.write().expect("rwlock should write");
        *value = String::from("v2");
    }

    println!(
        "final config = {}",
        config.read().expect("rwlock should read")
    );
}
