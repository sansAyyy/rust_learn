use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 锁粒度越大，代码越简单，但并发度可能更低。
    let global_counts = Arc::new(Mutex::new(HashMap::<String, usize>::new()));
    let texts = vec![
        "rust is fast",
        "rust is safe",
        "safe code is good",
        "fast code is fun",
    ];

    let mut handles = Vec::new();
    for text in texts {
        let counts = Arc::clone(&global_counts);
        handles.push(thread::spawn(move || {
            // 更好的方式：先在线程本地统计，再短暂持有锁合并结果。
            let mut local = HashMap::new();
            for word in text.split_whitespace() {
                *local.entry(word.to_string()).or_insert(0) += 1;
            }

            let mut global = counts.lock().expect("counts lock");
            for (word, count) in local {
                *global.entry(word).or_insert(0) += count;
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker should finish");
    }

    println!(
        "word counts = {:?}",
        global_counts.lock().expect("counts lock")
    );

    // 经验：不要在持有锁时执行慢操作、IO、网络请求或调用复杂外部代码。
}
