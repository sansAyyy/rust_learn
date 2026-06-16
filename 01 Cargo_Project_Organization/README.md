# Cargo 和项目组织

这一组文件是一个完整的 Cargo package，用来学习 Rust 项目结构、crate、模块、可见性、测试、文档和常用 Cargo 命令。

常用命令：

```powershell
cargo run
cargo run --bin 01_cargo_commands
cargo run --bin 02_crate_package_binary_library
cargo run --bin 03_modules_paths_visibility
cargo run --bin 04_use_and_reexport
cargo run --bin 05_dependencies_and_features
cargo run --bin 06_testing_docs_examples
cargo test
cargo doc --open
cargo fmt
cargo clippy
```

目录重点：

- `Cargo.toml`：包元数据、依赖、feature 配置。
- `src/main.rs`：默认二进制 crate 入口。
- `src/lib.rs`：库 crate 入口，可被 `main.rs`、`examples`、`tests` 使用。
- `src/bin/*.rs`：多个额外二进制程序，每个都有自己的 `fn main()`。
- `src/config.rs`、`src/math/*`、`src/models/*`：模块拆分示例。
- `examples/use_library.rs`：示例程序，像外部用户一样调用库。
- `tests/integration_tests.rs`：集成测试，像外部 crate 一样测试库 API。
