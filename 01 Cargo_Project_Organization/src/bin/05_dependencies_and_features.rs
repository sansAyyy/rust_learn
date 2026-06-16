fn main() {
    // 依赖写在 Cargo.toml：
    //
    // [dependencies]
    // serde = { version = "1", features = ["derive"] }
    // anyhow = "1"
    //
    // [dev-dependencies]
    // pretty_assertions = "1"

    println!("这个示例不引入外部依赖，方便离线编译。");

    // feature 是 Cargo 的条件编译开关。
    // 本项目 Cargo.toml 中定义了：
    // [features]
    // default = []
    // cli-output = []
    //
    // 运行：
    // cargo run --bin 05_dependencies_and_features
    // cargo run --bin 05_dependencies_and_features --features cli-output

    #[cfg(feature = "cli-output")]
    println!("feature cli-output is enabled");

    #[cfg(not(feature = "cli-output"))]
    println!("feature cli-output is disabled");

    // 常用依赖管理命令：
    // cargo add crate_name       添加依赖，需要安装 cargo-edit 或新版 Cargo 支持。
    // cargo update               更新 Cargo.lock 中允许更新的版本。
    // cargo tree                 查看依赖树。
}
