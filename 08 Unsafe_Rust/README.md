# unsafe Rust

`unsafe` 不会关闭借用检查，也不会让 Rust 变成“不安全语言”。它只是允许你使用少数编译器无法静态证明安全的能力。

unsafe Rust 主要允许五类操作：

- 解引用裸指针
- 调用 unsafe 函数或方法
- 访问或修改可变静态变量
- 实现 unsafe trait
- 访问 union 字段

常用命令：

```powershell
cargo run --bin 01_unsafe_superpowers
cargo run --bin 02_raw_pointers
cargo run --bin 03_unsafe_functions
cargo run --bin 04_safe_abstraction_split_at_mut
cargo run --bin 05_ffi_extern_c
cargo run --bin 06_mutable_static
cargo run --bin 07_unsafe_trait
cargo run --bin 08_unsafe_checklist
cargo test
```

学习原则：

- unsafe 块要尽量小。
- unsafe 代码必须写清楚调用者或实现者需要保证的不变量。
- 优先把 unsafe 封装在安全 API 后面。
- 能用安全 Rust 写，就不要用 unsafe。
