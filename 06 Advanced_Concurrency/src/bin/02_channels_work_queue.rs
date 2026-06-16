use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Job {
    id: usize,
    payload: String,
}

fn main() {
    // std::sync::mpsc 是多生产者、单消费者。
    // 如果想让多个 worker 共享同一个 receiver，需要用 Arc<Mutex<Receiver<T>>>。
    let (tx, rx) = mpsc::channel::<Job>();
    let rx = Arc::new(Mutex::new(rx));

    let mut workers = Vec::new();
    for worker_id in 1..=3 {
        let rx = Arc::clone(&rx);
        workers.push(thread::spawn(move || loop {
            let message = rx.lock().expect("receiver lock").recv();
            match message {
                Ok(job) => {
                    println!(
                        "worker {worker_id} got job {} with payload {}",
                        job.id, job.payload
                    );
                    thread::sleep(Duration::from_millis(30));
                }
                Err(_) => {
                    println!("worker {worker_id} exits");
                    break;
                }
            }
        }));
    }

    for id in 1..=6 {
        tx.send(Job {
            id,
            payload: format!("data-{id}"),
        })
        .expect("workers should receive jobs");
    }

    drop(tx); // 关闭发送端，worker 的 recv 会返回 Err。

    for worker in workers {
        worker.join().expect("worker should finish");
    }
}
