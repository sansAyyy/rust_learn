# 智能指针与内部可变性

这一组示例讲 Rust 中常见的“所有权组织工具”：

- `Box<T>`：把值放到堆上，常用于递归类型和 trait object
- `Rc<T>`：单线程多所有者共享
- `Arc<T>`：多线程多所有者共享
- `Cell<T>`：Copy 值的内部可变性
- `RefCell<T>`：运行时借用检查，单线程内部可变性
- `Mutex<T>` / `RwLock<T>`：多线程内部可变性
- `Weak<T>`：避免 `Rc`/`Arc` 引用循环
- `Cow<'a, T>`：写时克隆，借用和拥有之间的折中

常用命令：

```powershell
cargo run --bin 01_box_recursive_types
cargo run --bin 02_rc_shared_ownership
cargo run --bin 03_refcell_interior_mutability
cargo run --bin 04_rc_refcell_graph
cargo run --bin 05_weak_to_avoid_cycles
cargo run --bin 06_arc_mutex_threads
cargo run --bin 07_cell_and_cow
cargo run --bin 08_pointer_choice_guide
cargo test
```
