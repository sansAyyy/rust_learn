use cargo_project_organization::{add, crate_name};

fn main() {
    // package：由 Cargo.toml 描述的整个包。
    // crate：Rust 的编译单元，可以是 binary crate 或 library crate。
    // module：crate 内部组织代码的命名空间。

    // 当前 package 同时包含：
    // 1. src/lib.rs            library crate
    // 2. src/main.rs           默认 binary crate
    // 3. src/bin/*.rs          多个额外 binary crate

    println!("library crate name = {}", crate_name());
    println!("calling library function add(10, 20) = {}", add(10, 20));

    // 经验：
    // - 业务逻辑优先放到 lib.rs 暴露的模块里。
    // - main.rs 尽量薄，只负责解析输入、调用库、输出结果。
    // - 这样代码更容易测试，也更容易复用。
}
