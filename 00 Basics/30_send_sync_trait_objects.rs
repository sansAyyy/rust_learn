use std::sync::{Arc, Mutex};
use std::thread;

trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn run(&self);
}

struct PrintJob {
    name: String,
    message: String,
}

impl Job for PrintJob {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) {
        println!("{}: {}", self.name, self.message);
    }
}

fn main() {
    // Send：类型的所有权可以安全地在线程间转移。
    // Sync：多个线程可以安全地共享该类型的引用。
    // 大多数普通类型自动实现 Send/Sync；Rc、RefCell 等不是线程安全类型则不会。

    let job: Box<dyn Job> = Box::new(PrintJob {
        name: String::from("local"),
        message: String::from("runs on current thread"),
    });
    job.run();

    // 要把 trait object 放进线程，通常需要 dyn Trait + Send + Sync + 'static。
    let shared_job: Arc<dyn Job + Send + Sync + 'static> = Arc::new(PrintJob {
        name: String::from("shared"),
        message: String::from("runs from another thread"),
    });

    let cloned = Arc::clone(&shared_job);
    let handle = thread::spawn(move || {
        println!("thread got job: {}", cloned.name());
        cloned.run();
    });
    handle.join().expect("thread should finish");

    // Arc<Mutex<T>> 是多线程共享可变状态的常见组合。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = counter.lock().expect("mutex should not be poisoned");
            *guard += 1;
        }));
    }

    for handle in handles {
        handle.join().expect("worker should finish");
    }

    println!("counter = {}", *counter.lock().expect("mutex should lock"));

    run_job(shared_job);
}

fn run_job(job: Arc<dyn Job + Send + Sync + 'static>) {
    println!("run_job received: {}", job.name());
    job.run();
}
