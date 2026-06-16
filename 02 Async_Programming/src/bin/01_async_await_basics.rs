use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // async fn 返回一个 Future。Future 是惰性的，只有 .await 或被运行时轮询时才执行。
    let user_future = fetch_user(1);
    println!("future created, but not completed yet");

    let user = user_future.await;
    println!("user = {user}");

    // .await 会让当前任务暂停，把线程让给运行时执行其他任务。
    let profile = fetch_profile(&user).await;
    println!("profile = {profile}");

    // async 块也能创建 Future。
    let summary_future = async {
        let count = fetch_message_count().await;
        format!("{user} has {count} messages")
    };
    println!("{}", summary_future.await);

    print_future_type(fetch_message_count());
}

async fn fetch_user(id: u64) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("user-{id}")
}

async fn fetch_profile(user: &str) -> String {
    sleep(Duration::from_millis(80)).await;
    format!("profile for {user}")
}

async fn fetch_message_count() -> usize {
    sleep(Duration::from_millis(50)).await;
    12
}

fn print_future_type<F>(_: F)
where
    F: Future<Output = usize>,
{
    println!("async fn really returns a Future<Output = usize>");
}
