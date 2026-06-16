use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn main() {
    // async 块会创建一个 Future。Future 是惰性的：只有被轮询时才会执行。
    let future = async {
        let user = fetch_user().await;
        let posts = fetch_posts(&user).await;
        format!("{user} has {} posts", posts.len())
    };

    let summary = block_on(future);
    println!("{summary}");

    // async fn 的返回值类型是 impl Future<Output = T>。
    let number = block_on(add_async(20, 22));
    println!("async addition = {number}");
}

async fn fetch_user() -> String {
    // 真实项目里这里可能是网络请求、文件 IO 或数据库查询。
    String::from("Alice")
}

async fn fetch_posts(user: &str) -> Vec<String> {
    vec![
        format!("{user}'s first post"),
        format!("{user}'s second post"),
    ]
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

// 一个极简 executor，用标准库演示如何驱动 Future。
// 生产项目通常使用 tokio、async-std 或 smol 等运行时。
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Future::poll(Pin::as_mut(&mut future), &mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}

fn noop_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &NOOP_WAKER_VTABLE)
}

static NOOP_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    noop_clone,
    noop_wake,
    noop_wake_by_ref,
    noop_drop,
);

unsafe fn noop_clone(_: *const ()) -> RawWaker {
    noop_raw_waker()
}

unsafe fn noop_wake(_: *const ()) {}

unsafe fn noop_wake_by_ref(_: *const ()) {}

unsafe fn noop_drop(_: *const ()) {}
