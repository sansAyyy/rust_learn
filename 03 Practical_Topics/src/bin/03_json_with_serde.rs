use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: Option<String>,
    roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    data: T,
    request_id: String,
}

fn main() -> Result<()> {
    let user = User {
        id: 1,
        name: String::from("Alice"),
        email: Some(String::from("alice@example.com")),
        roles: vec![String::from("admin"), String::from("user")],
    };

    // 序列化：Rust 结构体 -> JSON 字符串。
    let json = serde_json::to_string_pretty(&user)?;
    println!("user json:\n{json}");

    // 反序列化：JSON 字符串 -> Rust 结构体。
    let parsed: User = serde_json::from_str(&json)?;
    println!("parsed user = {:?}", parsed);

    // 泛型结构体也可以序列化，只要内部类型实现 Serialize/Deserialize。
    let response = ApiResponse {
        data: parsed,
        request_id: String::from("req-001"),
    };
    println!("response = {}", serde_json::to_string_pretty(&response)?);

    // 也可以用 serde_json::Value 处理结构不固定的 JSON。
    let value: serde_json::Value = serde_json::json!({
        "ok": true,
        "count": 3,
        "tags": ["rust", "serde"]
    });
    println!("dynamic json count = {}", value["count"]);

    Ok(())
}
