# 宏与过程宏

这一章使用 Cargo workspace，因为过程宏必须定义在独立的 `proc-macro` crate 中。

目录：

- `macro_examples`：普通可运行示例，讲 `macro_rules!`、内置宏、以及如何使用过程宏。
- `learning_macros`：过程宏 crate，实现自定义 derive 宏、函数式过程宏、属性宏。

常用命令：

```powershell
cargo run -p macro_examples --bin 01_macro_rules_basics
cargo run -p macro_examples --bin 02_macro_rules_repetition
cargo run -p macro_examples --bin 03_builtin_macros
cargo run -p macro_examples --bin 04_using_proc_macros
cargo test
```

学习重点：

- `macro_rules!` 是声明宏，适合轻量代码生成。
- 过程宏处理 token stream，可以实现 `derive`、属性宏、函数式宏。
- 宏生成的是 Rust 代码，编译器随后继续类型检查这些代码。
- 不要为了少写几行普通函数就滥用宏；宏适合消除重复结构和创建小型 DSL。
