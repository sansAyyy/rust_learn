# 并发进阶

这一组示例聚焦标准库并发能力和常见工程模式：

- 线程创建、命名、join
- channel 消息传递
- 共享状态与锁粒度
- 原子类型与内存顺序
- `Send` / `Sync` 边界
- 简易线程池
- 并行 map/reduce
- 死锁风险与规避策略

常用命令：

```powershell
cargo run --bin 01_thread_builder_and_join
cargo run --bin 02_channels_work_queue
cargo run --bin 03_shared_state_lock_granularity
cargo run --bin 04_atomics
cargo run --bin 05_send_sync_boundaries
cargo run --bin 06_simple_thread_pool
cargo run --bin 07_parallel_map_reduce
cargo run --bin 08_deadlock_avoidance
cargo test
```
