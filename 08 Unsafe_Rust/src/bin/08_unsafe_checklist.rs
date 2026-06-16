fn main() {
    println!("unsafe 使用清单：");
    println!("1. unsafe 块是否尽可能小？");
    println!("2. 是否写清楚 Safety 不变量？");
    println!("3. 是否能用安全 Rust 替代？");
    println!("4. 裸指针是否非空、对齐、有效、未越界？");
    println!("5. 是否避免了别名规则冲突？");
    println!("6. FFI 边界是否明确内存由谁分配和释放？");
    println!("7. 全局可变状态是否使用 Atomic/Mutex/OnceLock 替代 static mut？");
    println!("8. unsafe 是否被封装成安全 API，并配有测试？");
}
