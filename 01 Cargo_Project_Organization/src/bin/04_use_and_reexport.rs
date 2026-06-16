// use 只是把路径引入当前作用域，不会改变所有权或可见性。
use cargo_project_organization::math::stats;

// pub use 在库代码中常用于重新导出，让用户少写路径。
// 本项目的 lib.rs 已经把 Config、User、add 重新导出到了 crate 根。
use cargo_project_organization::{add, Config, User};

fn main() {
    let config = Config::new("local", true);
    let user = User::new(42, "Carol");

    println!("config = {:?}", config);
    println!("user = {}", user.display_name());
    println!("add through re-export = {}", add(1, 2));

    let scores = [80, 92, 75, 99];
    println!("max score = {:?}", stats::max(&scores));

    // 常见实践：
    // - 标准库类型常直接 use，例如 `use std::collections::HashMap;`
    // - 本地模块可以保留较完整路径，增强可读性。
    // - 对外 API 可以用 pub use 做“门面”，隐藏内部文件布局。
}
