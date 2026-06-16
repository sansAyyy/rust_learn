use std::borrow::Cow;
use std::cell::Cell;

struct Counter {
    value: Cell<u32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }

    fn increment(&self) {
        // Cell<T> 适合 Copy 类型。它通过 get/set 修改内部值，不返回引用。
        self.value.set(self.value.get() + 1);
    }

    fn get(&self) -> u32 {
        self.value.get()
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("counter = {}", counter.get());

    // Cow<'a, str> 可以是借用的 &str，也可以是拥有的 String。
    // 只有确实需要修改时才分配新 String。
    let clean = normalize("Alice");
    let changed = normalize("  Bob Smith  ");
    println!("clean = {clean}");
    println!("changed = {changed}");

    let mut text = Cow::Borrowed("hello");
    text.to_mut().push_str(" world");
    println!("cow after mutation = {text}");
}

fn normalize(input: &str) -> Cow<'_, str> {
    let trimmed = input.trim();
    if trimmed.contains(' ') {
        Cow::Owned(trimmed.replace(' ', "_").to_lowercase())
    } else {
        Cow::Borrowed(trimmed)
    }
}
