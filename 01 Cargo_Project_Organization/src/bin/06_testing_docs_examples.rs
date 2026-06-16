use cargo_project_organization::math::stats;

fn main() {
    // Rust 常见测试位置：
    // 1. 单元测试：和代码放在同一个文件，用 #[cfg(test)] mod tests。
    // 2. 集成测试：放在 tests/ 目录，每个文件像外部 crate 一样测试 public API。
    // 3. 文档测试：写在 /// 或 //! 文档注释里的 ```rust 代码块。

    let values = [1.0, 2.0, 3.0, 4.0];
    println!("average = {:?}", stats::average(&values));

    // 运行全部测试：
    // cargo test
    //
    // 只运行名称包含 average 的测试：
    // cargo test average
    //
    // 生成文档：
    // cargo doc --open

    assert_eq!(stats::average(&values), Some(2.5));
}
