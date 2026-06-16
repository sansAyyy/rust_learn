use std::thread;
use std::time::Duration;

fn main() {
    // thread::Builder 可以设置线程名和栈大小。
    let handle = thread::Builder::new()
        .name(String::from("worker-a"))
        .stack_size(2 * 1024 * 1024)
        .spawn(|| {
            let current = thread::current();
            let name = current.name().unwrap_or("unnamed");
            for index in 1..=3 {
                println!("{name}: step {index}");
                thread::sleep(Duration::from_millis(50));
            }
            42
        })
        .expect("thread should spawn");

    // 主线程可以同时做其他工作。
    println!("main thread keeps working");

    // join 会等待线程完成，并取回返回值。
    let result = handle.join().expect("worker should not panic");
    println!("worker returned {result}");

    // scoped thread 可以借用当前栈上的数据，不要求 'static。
    let numbers = vec![1, 2, 3, 4];
    thread::scope(|scope| {
        scope.spawn(|| {
            let sum: i32 = numbers.iter().sum();
            println!("scoped thread sum = {sum}");
        });
    });
}
