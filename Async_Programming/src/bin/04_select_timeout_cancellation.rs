use std::time::Duration;
use tokio::time::{interval, sleep, timeout};

#[tokio::main]
async fn main() {
    // timeout 给一个 Future 加上最长等待时间。
    match timeout(Duration::from_millis(100), slow_operation()).await {
        Ok(value) => println!("slow operation completed: {value}"),
        Err(_) => println!("slow operation timed out"),
    }

    // select! 同时等待多个异步分支，谁先完成就执行谁。
    tokio::select! {
        value = fast_operation() => {
            println!("fast branch won: {value}");
        }
        value = slow_operation() => {
            println!("slow branch won: {value}");
        }
    }

    // 未完成的 select 分支会被取消，也就是对应 Future 会被丢弃。
    cancellation_demo().await;

    // interval 常用于周期性任务。
    let mut ticker = interval(Duration::from_millis(100));
    for index in 1..=3 {
        ticker.tick().await;
        println!("tick {index}");
    }
}

async fn fast_operation() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "fast"
}

async fn slow_operation() -> &'static str {
    sleep(Duration::from_millis(300)).await;
    "slow"
}

async fn cancellation_demo() {
    let long_task = async {
        sleep(Duration::from_secs(5)).await;
        "long task finished"
    };

    tokio::select! {
        result = long_task => {
            println!("{result}");
        }
        _ = sleep(Duration::from_millis(80)) => {
            println!("long task was cancelled by select");
        }
    }
}
