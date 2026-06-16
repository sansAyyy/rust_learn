use anyhow::Result;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<()> {
    // 标准库线程适合 CPU 工作或简单并发。
    // channel 用于在线程之间传递消息，避免共享可变状态。
    let (tx, rx) = mpsc::channel();
    let inputs = vec![30, 31, 32, 33];

    for input in inputs {
        let tx = tx.clone();
        thread::spawn(move || {
            let value = fibonacci(input);
            tx.send((input, value)).expect("receiver should exist");
        });
    }

    drop(tx);

    let mut results: Vec<_> = rx.into_iter().collect();
    results.sort_by_key(|(input, _)| *input);

    for (input, value) in results {
        println!("fib({input}) = {value}");
    }

    Ok(())
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
