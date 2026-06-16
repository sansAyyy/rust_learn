use cargo_project_organization::{add, Config, User};

fn main() {
    // examples/ 中的程序像外部用户一样使用当前库。
    // 运行：
    // cargo run --example use_library

    let config = Config::new("example", true);
    let user = User::new(100, "Example User");

    println!("config = {:?}", config);
    println!("user = {}", user.display_name());
    println!("add(8, 9) = {}", add(8, 9));
}
