use cargo_project_organization::{add, Config, User};

fn main() {
    // src/main.rs 是默认二进制 crate 的入口。
    // 它可以通过 package name 使用同包中的库 crate。
    let config = Config::new("development", true);
    let user = User::new(1, "Alice");

    println!("running with config: {:?}", config);
    println!("current user: {}", user.display_name());
    println!("2 + 3 = {}", add(2, 3));
}
