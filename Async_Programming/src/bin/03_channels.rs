use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // mpsc：multiple producer, single consumer。适合多个任务向一个任务发送消息。
    let (tx, mut rx) = mpsc::channel::<String>(8);

    for worker_id in 1..=3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for job_id in 1..=2 {
                let message = format!("worker {worker_id} produced job {job_id}");
                tx.send(message).await.expect("receiver should still exist");
                sleep(Duration::from_millis(30)).await;
            }
        });
    }

    drop(tx); // 关闭原始发送端；所有克隆发送端结束后，rx.recv() 会返回 None。

    while let Some(message) = rx.recv().await {
        println!("received: {message}");
    }

    // oneshot：只发送一次结果，常用于请求-响应。
    let (response_tx, response_rx) = oneshot::channel();
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        response_tx
            .send(String::from("single response"))
            .expect("receiver should still exist");
    });

    let response = response_rx.await.expect("sender should send a response");
    println!("oneshot response = {response}");
}
