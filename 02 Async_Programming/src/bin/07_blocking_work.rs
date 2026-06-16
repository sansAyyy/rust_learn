use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // 异步任务不应该直接执行长时间阻塞 CPU 或线程的操作。
    // 对阻塞代码使用 spawn_blocking，把它交给专门的阻塞线程池。
    let handle = tokio::task::spawn_blocking(|| expensive_cpu_work(35));

    // 在 CPU 工作进行时，异步运行时仍能执行其他任务。
    for tick in 1..=3 {
        sleep(Duration::from_millis(100)).await;
        println!("async tick {tick}");
    }

    let result = handle.await.expect("blocking task should not panic");
    println!("expensive result = {result}");

    // 如果只是想让出执行权，可以用 yield_now。
    tokio::task::yield_now().await;
    println!("yielded back to runtime once");
}

fn expensive_cpu_work(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => expensive_cpu_work(n - 1) + expensive_cpu_work(n - 2),
    }
}
