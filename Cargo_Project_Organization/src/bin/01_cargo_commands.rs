fn main() {
    // 这个文件位于 src/bin/，因此它是一个额外的二进制 crate。
    // 运行：
    // cargo run --bin 01_cargo_commands

    println!("Cargo 常用命令：");
    println!("cargo new app_name      创建新项目");
    println!("cargo init              在当前目录初始化项目");
    println!("cargo build             编译 debug 版本");
    println!("cargo build --release   编译 release 优化版本");
    println!("cargo run               编译并运行默认二进制");
    println!("cargo test              运行测试");
    println!("cargo check             快速类型检查，不生成最终二进制");
    println!("cargo fmt               格式化代码");
    println!("cargo clippy            运行 lint 检查");
    println!("cargo doc --open        生成并打开文档");

    // target/ 是 Cargo 的构建输出目录，通常不提交到 Git。
    // Cargo.lock 记录精确依赖版本；应用项目通常提交，库项目视情况而定。
}
