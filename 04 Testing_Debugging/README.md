# 测试与调试

这一组示例演示 Rust 项目中常用的测试与调试方式：

- 单元测试：和被测代码放在同一个文件中
- 集成测试：放在 `tests/` 目录
- 文档测试：写在文档注释里的代码块
- `Result` 返回值测试
- `#[should_panic]`
- 测试辅助模块
- 临时文件测试
- `dbg!`、`eprintln!`
- `tracing` 日志调试
- `cargo fmt`、`cargo clippy`、`cargo test`

常用命令：

```powershell
cargo test
cargo test calculator
cargo test -- --nocapture
cargo test --doc
cargo run --bin 01_debug_macros
cargo run --bin 02_logging_debug
cargo fmt
cargo clippy
```
