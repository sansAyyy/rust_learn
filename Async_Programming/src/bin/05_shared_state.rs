use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

#[tokio::main]
async fn main() {
    // Arc 让多个任务共享同一个所有权。
    // tokio::sync::Mutex 的 lock 是 async，不会阻塞运行时线程。
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut guard = counter.lock().await;
            *guard += 1;
            // guard 离开作用域时自动解锁。
        }));
    }

    for handle in handles {
        handle.await.expect("counter task should finish");
    }

    println!("counter = {}", *counter.lock().await);

    // 不要在持有锁时执行耗时 .await。通常用作用域提前释放锁。
    let names = Arc::new(Mutex::new(Vec::<String>::new()));
    {
        let mut guard = names.lock().await;
        guard.push(String::from("Alice"));
    } // 锁在这里释放。
    async_work_after_unlock().await;
    println!("names = {:?}", names.lock().await);

    // RwLock 允许多个读者或一个写者。
    let config = Arc::new(RwLock::new(String::from("v1")));
    let read_a = read_config(Arc::clone(&config));
    let read_b = read_config(Arc::clone(&config));
    let write = write_config(Arc::clone(&config), String::from("v2"));

    tokio::join!(read_a, read_b, write);
    println!("final config = {}", config.read().await);
}

async fn async_work_after_unlock() {
    tokio::task::yield_now().await;
}

async fn read_config(config: Arc<RwLock<String>>) {
    let value = config.read().await;
    println!("read config = {value}");
}

async fn write_config(config: Arc<RwLock<String>>, value: String) {
    let mut guard = config.write().await;
    *guard = value;
    println!("config updated");
}
