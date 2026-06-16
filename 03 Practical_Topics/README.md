# 常用实战主题

这一组示例是从“语言概念”走向“能写小工具”的过渡。它使用真实项目常见 crate：

- `clap`：命令行参数解析
- `serde` / `serde_json`：序列化和反序列化
- `reqwest`：HTTP 客户端
- `tracing` / `tracing-subscriber`：结构化日志
- `toml`：配置文件解析
- `anyhow`：应用层错误处理

运行示例：

```powershell
cargo run --bin 01_cli_with_clap -- greet Alice --shout
cargo run --bin 02_file_io_and_paths
cargo run --bin 03_json_with_serde
cargo run --bin 04_http_client_reqwest
cargo run --bin 05_logging_with_tracing
cargo run --bin 06_config_files_and_env
cargo run --bin 07_small_todo_cli add "learn rust"
cargo run --bin 08_threads_channels_parallel
cargo test
```

`04_http_client_reqwest` 默认不发真实网络请求。要实际请求，可以设置环境变量：

```powershell
$env:RUN_HTTP_EXAMPLE="1"
cargo run --bin 04_http_client_reqwest
```
