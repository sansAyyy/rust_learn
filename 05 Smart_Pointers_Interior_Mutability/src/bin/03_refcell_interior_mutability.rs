use std::cell::RefCell;

trait Messenger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent = self.value as f64 / self.max as f64;

        if percent >= 1.0 {
            self.messenger.send("error: quota exceeded");
        } else if percent >= 0.9 {
            self.messenger.send("urgent warning: quota above 90%");
        } else if percent >= 0.75 {
            self.messenger.send("warning: quota above 75%");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> Self {
        Self {
            sent_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // RefCell<T> 允许在只有 &self 时修改内部数据。
        // 借用规则从编译期移动到运行时检查，违规会 panic。
        self.sent_messages.borrow_mut().push(message.to_string());
    }
}

fn main() {
    let messenger = MockMessenger::new();
    let mut tracker = LimitTracker::new(&messenger, 100);

    tracker.set_value(80);
    tracker.set_value(95);

    let messages = messenger.sent_messages.borrow();
    println!("messages = {:?}", *messages);

    drop(messages);

    // try_borrow_mut 返回 Result，比 borrow_mut panic 更适合演示和恢复。
    let first_borrow = messenger.sent_messages.borrow();
    let second_borrow = messenger.sent_messages.try_borrow_mut();
    println!(
        "can borrow mut while immutable borrow exists? {}",
        second_borrow.is_ok()
    );
    drop(first_borrow);

    messenger
        .sent_messages
        .borrow_mut()
        .push(String::from("manual message after borrow released"));
    println!("final messages = {:?}", messenger.sent_messages.borrow());
}
