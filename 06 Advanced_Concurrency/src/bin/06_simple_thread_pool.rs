use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Run(Job),
    Shutdown,
}

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "thread pool size must be positive");

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(thread::spawn(move || loop {
                let message = receiver.lock().expect("receiver lock").recv();
                match message {
                    Ok(Message::Run(job)) => {
                        println!("worker {id} runs a job");
                        job();
                    }
                    Ok(Message::Shutdown) | Err(_) => {
                        println!("worker {id} shuts down");
                        break;
                    }
                }
            }));
        }

        Self { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .send(Message::Run(Box::new(f)))
            .expect("workers should receive jobs");
    }

    fn shutdown(self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Shutdown)
                .expect("workers should receive shutdown");
        }

        for worker in self.workers {
            worker.join().expect("worker should finish");
        }
    }
}

fn main() {
    let pool = ThreadPool::new(3);

    for job_id in 1..=6 {
        pool.execute(move || {
            let result = job_id * job_id;
            println!("job {job_id} result = {result}");
        });
    }

    pool.shutdown();
}
