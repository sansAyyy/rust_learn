use std::rc::Rc;
use std::sync::Arc;
use std::thread;

trait Handler: Send + Sync {
    fn handle(&self, input: &str) -> String;
}

struct EchoHandler;

impl Handler for EchoHandler {
    fn handle(&self, input: &str) -> String {
        format!("echo: {input}")
    }
}

fn main() {
    // Rc<T> 不是 Send/Sync，不能跨线程共享。
    let local = Rc::new(String::from("single-thread only"));
    println!("local rc = {local}");

    // Arc<T> 可以跨线程共享，只要 T 本身满足 Send/Sync。
    let shared = Arc::new(String::from("shared across threads"));
    let cloned = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        println!("thread sees: {cloned}");
    });
    handle.join().expect("thread should finish");

    // trait object 跨线程时，把边界写清楚：dyn Trait + Send + Sync + 'static。
    let handler: Arc<dyn Handler + Send + Sync + 'static> = Arc::new(EchoHandler);
    let handler_clone = Arc::clone(&handler);
    let handle = thread::spawn(move || {
        println!("{}", handler_clone.handle("hello"));
    });
    handle.join().expect("handler thread should finish");

    require_send_sync(handler);
}

fn require_send_sync<T>(_value: T)
where
    T: Send + Sync + 'static,
{
    println!("value satisfies Send + Sync + 'static");
}
