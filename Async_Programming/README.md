# Rust 异步编程

这一组示例使用 Tokio 学习真实项目中常见的异步 Rust 写法。

常用命令：

```powershell
cargo run
cargo run --bin 01_async_await_basics
cargo run --bin 02_join_and_spawn
cargo run --bin 03_channels
cargo run --bin 04_select_timeout_cancellation
cargo run --bin 05_shared_state
cargo run --bin 06_async_error_handling
cargo run --bin 07_blocking_work
cargo test
```

学习顺序：

- `01_async_await_basics.rs`：`async fn`、`Future`、`.await`
- `02_join_and_spawn.rs`：并发等待、任务、`JoinHandle`
- `03_channels.rs`：`mpsc`、`oneshot`
- `04_select_timeout_cancellation.rs`：`select!`、timeout、取消
- `05_shared_state.rs`：`Arc`、`Mutex`、`RwLock`
- `06_async_error_handling.rs`：异步函数中的 `Result`、任务错误处理
- `07_blocking_work.rs`：`spawn_blocking`、避免阻塞异步运行时
