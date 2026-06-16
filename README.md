# Rust Learn

这是一个按学习阶段整理的 Rust 示例仓库。目标不是一次性写出完整项目，而是用一组可运行的小例子，把 Rust 的核心概念、项目组织和异步编程逐步练熟。

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
└── 02 Async_Programming
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

## 建议学习方式

1. 先按编号阅读 `00 Basics`，每个文件运行一次。
2. 遇到注释中的“编译错误示例”，可以临时取消注释，观察编译器提示。
3. 学完基础语法后，进入 `01 Cargo_Project_Organization`，理解真实项目如何组织代码。
4. 再进入 `02 Async_Programming`，学习运行时、任务、并发和异步错误处理。
5. 每学完一组，尝试把示例改一点，例如换输入、加字段、增加错误分支、拆模块。

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

这个仓库已经覆盖 Rust 入门到进阶前半段的主要主题。下一步适合继续补充：

- 命令行工具实战
- 文件 IO 与 JSON
- HTTP 客户端/服务端
- 数据库访问
- 测试策略与 mock
- 宏与过程宏
- unsafe Rust 入门
