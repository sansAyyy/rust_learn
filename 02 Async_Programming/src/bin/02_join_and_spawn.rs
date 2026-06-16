use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // 顺序 await：第二个请求会等第一个请求完成后才开始。
    let start = Instant::now();
    let a = fetch("A", 200).await;
    let b = fetch("B", 200).await;
    println!("sequential: {a}, {b}, elapsed = {:?}", start.elapsed());

    // tokio::join!：多个 Future 同时推进，在同一个任务中并发执行。
    let start = Instant::now();
    let (a, b) = tokio::join!(fetch("A", 200), fetch("B", 200));
    println!("join: {a}, {b}, elapsed = {:?}", start.elapsed());

    // tokio::spawn：把 Future 放到运行时中作为独立任务。
    // 被 spawn 的任务通常要求 Send + 'static，因为它可能被调度到其他线程。
    let name = String::from("background");
    let handle = tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        format!("{name} task finished")
    });

    do_other_work().await;

    // JoinHandle.await 返回 Result，任务 panic 或被取消时会是 Err。
    let result = handle.await.expect("task should not panic");
    println!("{result}");
}

async fn fetch(label: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("response-{label}")
}

async fn do_other_work() {
    println!("main task is doing other work");
    sleep(Duration::from_millis(50)).await;
}
