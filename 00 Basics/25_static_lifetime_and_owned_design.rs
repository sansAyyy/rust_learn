use std::borrow::Cow;

fn main() {
    // &'static str 表示引用的数据在整个程序运行期间都有效。
    let literal: &'static str = "string literals are static";
    println!("{literal}");

    // 'static 不等于“永远不释放所有权类型”。
    // String 本身可以被移动和释放；只有字符串字面量这种数据天然是 static。
    let owned = String::from("owned string");
    print_message(&owned);
    println!("owned still lives here = {owned}");

    // 需要长期保存或跨线程移动时，优先考虑拥有数据。
    let job = Job::new("backup", String::from("run at midnight"));
    println!("job = {:?}", job);
    println!("job '{}' command length = {}", job.name, job.command.len());

    // Cow 可以在“借用或拥有”之间折中：没有修改时借用，需要修改时变成拥有。
    let clean = normalize_username("Alice");
    let changed = normalize_username("  Bob Smith  ");
    println!("clean = {clean}, changed = {changed}");

    // thread::spawn 要求闭包通常是 'static，因为线程可能比当前栈帧活得更久。
    let message = String::from("hello from thread");
    let handle = std::thread::spawn(move || {
        // move 把 message 所有权移动进线程闭包，满足 'static 要求。
        println!("{message}");
    });
    handle.join().expect("thread should finish");

    // Box::leak 可以把堆数据泄漏成 &'static mut T，但这是有意不释放内存。
    // 一般只在全局配置、测试或特殊场景使用。
    let leaked: &'static str = Box::leak(String::from("leaked for program lifetime").into_boxed_str());
    println!("{leaked}");
}

fn print_message(message: &str) {
    println!("message = {message}");
}

#[derive(Debug)]
struct Job {
    name: &'static str,
    command: String,
}

impl Job {
    fn new(name: &'static str, command: String) -> Self {
        Self { name, command }
    }
}

fn normalize_username(input: &str) -> Cow<'_, str> {
    let trimmed = input.trim();
    if trimmed.contains(' ') {
        Cow::Owned(trimmed.replace(' ', "_").to_lowercase())
    } else {
        Cow::Borrowed(trimmed)
    }
}
