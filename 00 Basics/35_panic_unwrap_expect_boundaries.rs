fn main() {
    // panic 表示程序进入了不应该继续执行的状态。
    // Result 表示调用者可以理解并处理的失败。

    let config = "host=localhost";
    let host = required_value(config, "host").expect("sample config should contain host");
    println!("host = {host}");

    // unwrap 适合临时原型、测试、示例；工程代码中通常 prefer expect 或 ?。
    let number: i32 = "42".parse().unwrap();
    println!("number = {number}");

    // expect 比 unwrap 更好，因为 panic 信息说明了开发者的假设。
    let port: u16 = "8080"
        .parse()
        .expect("hard-coded port 8080 should parse as u16");
    println!("port = {port}");

    // assert 用于维护内部不变量，失败说明代码逻辑错了。
    let workers = 4;
    assert!(workers > 0, "worker count must be positive");

    // debug_assert 只在 debug 构建默认启用，适合开发期检查。
    debug_assert_eq!(workers * 2, 8);

    match divide(10, 2) {
        Ok(value) => println!("10 / 2 = {value}"),
        Err(error) => println!("division failed: {error}"),
    }

    // divide(10, 0).unwrap(); // 用户输入导致的错误，不应该 unwrap。
}

fn required_value<'a>(content: &'a str, key: &str) -> Result<&'a str, String> {
    for line in content.lines() {
        let Some((current_key, value)) = line.split_once('=') else {
            continue;
        };

        if current_key.trim() == key {
            return Ok(value.trim());
        }
    }

    Err(format!("missing required key '{key}'"))
}

fn divide(left: i32, right: i32) -> Result<i32, String> {
    if right == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(left / right)
    }
}
