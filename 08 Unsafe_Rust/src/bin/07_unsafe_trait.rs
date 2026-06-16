unsafe trait TrustedLength {
    fn trusted_len(&self) -> usize;
}

struct FixedItems<T> {
    items: Vec<T>,
}

unsafe impl<T> TrustedLength for FixedItems<T> {
    fn trusted_len(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let items = FixedItems {
        items: vec![1, 2, 3],
    };

    println!("trusted len = {}", items.trusted_len());

    // unsafe trait 表示实现者必须保证编译器无法检查的不变量。
    // 标准库中的 Send/Sync 就是 unsafe trait，不过很多类型会自动实现。
}
