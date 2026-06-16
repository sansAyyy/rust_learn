fn main() {
    println!("常用测试与调试命令：");
    println!("cargo test                         运行全部测试");
    println!("cargo test calculator              只运行名称包含 calculator 的测试");
    println!("cargo test -- --nocapture          显示测试中的 println! 输出");
    println!("cargo test --doc                   只运行文档测试");
    println!("cargo test --test integration_flow 运行指定集成测试文件");
    println!("cargo fmt                          格式化代码");
    println!("cargo clippy                       静态 lint 检查");
    println!("RUST_BACKTRACE=1 cargo test        panic 时显示回溯");
}
