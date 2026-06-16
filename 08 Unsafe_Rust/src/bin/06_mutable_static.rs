use std::sync::atomic::{AtomicUsize, Ordering};

static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

static mut LEGACY_COUNTER: usize = 0;

fn main() {
    // 推荐：用原子类型或锁管理全局可变状态。
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
    REQUEST_COUNT.fetch_add(1, Ordering::Relaxed);
    println!("request count = {}", REQUEST_COUNT.load(Ordering::Relaxed));

    // static mut 是 unsafe，因为多线程读写可能数据竞争。
    unsafe {
        LEGACY_COUNTER += 1;
        let value = LEGACY_COUNTER;
        println!("legacy counter = {value}");
    }

    // 真实项目里优先用 Atomic*、Mutex、OnceLock、LazyLock。
}
