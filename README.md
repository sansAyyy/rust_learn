# Rust Learn

这是一个按学习阶段整理的 Rust 示例仓库。目标不是一次性写出完整项目，而是用一组可运行的小例子，把 Rust 的核心概念、项目组织、异步编程、常用实战主题、测试与调试、智能指针、内部可变性、并发进阶、宏和 unsafe Rust 逐步练熟。

## 目录结构

```text
.
├── 00 Basics
│   ├── 01_variables_and_mutability.rs
│   ├── ...
│   └── 35_panic_unwrap_expect_boundaries.rs
├── 01 Cargo_Project_Organization
│   ├── Cargo.toml
│   ├── src
│   ├── examples
│   └── tests
├── 02 Async_Programming
│   ├── Cargo.toml
│   └── src/bin
├── 03 Practical_Topics
│   ├── Cargo.toml
│   └── src/bin
├── 04 Testing_Debugging
│   ├── Cargo.toml
│   ├── src
│   └── tests
├── 05 Smart_Pointers_Interior_Mutability
│   ├── Cargo.toml
│   └── src/bin
├── 06 Advanced_Concurrency
│   ├── Cargo.toml
│   └── src/bin
├── 07 Macros_Proc_Macros
│   ├── Cargo.toml
│   ├── macro_examples
│   └── learning_macros
└── 08 Unsafe_Rust
    ├── Cargo.toml
    └── src/bin
```

## 学习路线

### 00 Basics

单文件 Rust 示例，每个文件都可以单独编译运行。内容覆盖：

- 变量、数据类型、函数、流程控制
- 所有权、借用、引用、切片、生命周期
- 结构体、枚举、模式匹配
- 集合、模块、错误处理、泛型、Trait
- 闭包、迭代器、`Option` / `Result` 进阶
- 更深入的所有权与生命周期
- Trait 体系进阶
- 工程化错误处理

运行示例：

```powershell
cd "00 Basics"
rustc --edition=2021 01_variables_and_mutability.rs
.\01_variables_and_mutability.exe
```

也可以直接编译某个主题：

```powershell
rustc --edition=2021 30_send_sync_trait_objects.rs
.\30_send_sync_trait_objects.exe
```

### 01 Cargo_Project_Organization

完整 Cargo 项目，用来学习 Rust 工程结构：

- `Cargo.toml`
- package、crate、module
- `src/main.rs` 与 `src/lib.rs`
- `src/bin/*.rs`
- `examples`
- `tests`
- `pub`、`pub(crate)`、`pub use`
- 单元测试、集成测试、文档测试

运行示例：

```powershell
cd "01 Cargo_Project_Organization"
cargo run
cargo run --bin 01_cargo_commands
cargo test
cargo doc --open
```

### 02 Async_Programming

基于 Tokio 的异步 Rust 示例。内容覆盖：

- `async fn`、`Future`、`.await`
- `tokio::join!`
- `tokio::spawn`
- channel：`mpsc`、`oneshot`
- `select!`、timeout、取消
- `Arc`、`Mutex`、`RwLock`
- 异步错误处理
- `spawn_blocking`

运行示例：

```powershell
cd "02 Async_Programming"
cargo run --bin 01_async_await_basics
cargo run --bin 02_join_and_spawn
cargo test
```

### 03 Practical_Topics

常用实战主题示例，使用真实项目里常见的第三方 crate。内容覆盖：

- `clap` 命令行参数解析
- 文件 IO 与路径处理
- `serde` / `serde_json`
- `reqwest` HTTP 客户端
- `tracing` 结构化日志
- TOML 配置与环境变量覆盖
- JSON 持久化的小 todo CLI
- 标准库线程与 channel 并发

运行示例：

```powershell
cd "03 Practical_Topics"
cargo run --bin 01_cli_with_clap -- greet Alice --shout
cargo run --bin 03_json_with_serde
cargo run --bin 07_small_todo_cli -- add "learn rust"
cargo test
```

### 04 Testing_Debugging

测试与调试示例项目，覆盖 Rust 项目里常见的质量保障手段：

- 单元测试
- 集成测试
- 文档测试
- `Result` 返回值测试
- `#[should_panic]`
- 测试辅助模块
- 临时文件测试
- `dbg!`、`eprintln!`
- `tracing` 日志调试
- `cargo fmt`、`cargo clippy`、`cargo test`

运行示例：

```powershell
cd "04 Testing_Debugging"
cargo test
cargo test -- --nocapture
cargo run --bin 01_debug_macros
cargo run --bin 02_logging_debug
```

### 05 Smart_Pointers_Interior_Mutability

智能指针与内部可变性示例项目，覆盖复杂所有权结构中常用的工具：

- `Box<T>`
- `Rc<T>`
- `Arc<T>`
- `Cell<T>`
- `RefCell<T>`
- `Mutex<T>` / `RwLock<T>`
- `Weak<T>`
- `Cow<'a, T>`
- `Rc<RefCell<T>>`
- `Arc<Mutex<T>>`

运行示例：

```powershell
cd "05 Smart_Pointers_Interior_Mutability"
cargo run --bin 01_box_recursive_types
cargo run --bin 03_refcell_interior_mutability
cargo run --bin 06_arc_mutex_threads
cargo run --bin 08_pointer_choice_guide
cargo test
```

### 06 Advanced_Concurrency

并发进阶示例项目，覆盖标准库并发能力和常见工程模式：

- 线程创建、命名、`join`
- scoped thread
- channel 工作队列
- 共享状态与锁粒度
- 原子类型
- `Send` / `Sync` 边界
- 简易线程池
- 并行 map/reduce
- 死锁风险与规避策略

运行示例：

```powershell
cd "06 Advanced_Concurrency"
cargo run --bin 01_thread_builder_and_join
cargo run --bin 02_channels_work_queue
cargo run --bin 06_simple_thread_pool
cargo run --bin 08_deadlock_avoidance
cargo test
```

### 07 Macros_Proc_Macros

宏与过程宏 workspace，覆盖声明宏和三类过程宏：

- `macro_rules!`
- 宏重复匹配
- 常见内置宏
- 自定义 derive 宏
- 函数式过程宏
- 属性宏
- `syn` / `quote` / `proc_macro`

运行示例：

```powershell
cd "07 Macros_Proc_Macros"
cargo run -p macro_examples --bin 01_macro_rules_basics
cargo run -p macro_examples --bin 02_macro_rules_repetition
cargo run -p macro_examples --bin 03_builtin_macros
cargo run -p macro_examples --bin 04_using_proc_macros
cargo test
```

### 08 Unsafe_Rust

unsafe Rust 示例项目，重点学习如何识别 unsafe 边界，以及如何把 unsafe 封装成安全 API：

- unsafe 的五类能力
- 裸指针
- unsafe 函数
- 安全抽象封装
- FFI / `extern "C"`
- 可变静态变量
- unsafe trait
- unsafe 使用清单

运行示例：

```powershell
cd "08 Unsafe_Rust"
cargo run --bin 01_unsafe_superpowers
cargo run --bin 04_safe_abstraction_split_at_mut
cargo run --bin 05_ffi_extern_c
cargo run --bin 08_unsafe_checklist
cargo test
```

## 建议学习方式

1. 先按编号阅读 `00 Basics`，每个文件运行一次。
2. 遇到注释中的“编译错误示例”，可以临时取消注释，观察编译器提示。
3. 学完基础语法后，进入 `01 Cargo_Project_Organization`，理解真实项目如何组织代码。
4. 再进入 `02 Async_Programming`，学习运行时、任务、并发和异步错误处理。
5. 最后进入 `03 Practical_Topics`，把语言能力落到 CLI、文件、JSON、HTTP、日志和配置这些常见任务上。
6. 用 `04 Testing_Debugging` 学会给代码加测试、看错误、打日志和跑工具链检查。
7. 用 `05 Smart_Pointers_Interior_Mutability` 学习复杂数据结构中的共享所有权和内部可变性。
8. 用 `06 Advanced_Concurrency` 深入理解线程、消息传递、锁、原子类型和并发任务拆分。
9. 用 `07 Macros_Proc_Macros` 理解 `println!`、`vec!`、`derive` 和属性宏背后的代码生成机制。
10. 用 `08 Unsafe_Rust` 学会识别 unsafe 边界，并练习把 unsafe 包进安全抽象。
11. 每学完一组，尝试把示例改一点，例如换输入、加字段、增加错误分支、拆模块。

## 常用命令

```powershell
rustc --version
cargo --version
cargo fmt
cargo clippy
cargo test
cargo run
```

## 当前阶段

这个仓库已经覆盖 Rust 入门到进阶阶段的大部分语言与工程主题。下一步适合继续补充完整项目实战：

- 更完整的命令行项目
- HTTP 服务端
- 数据库访问
- 更系统的 mock 与测试替身
