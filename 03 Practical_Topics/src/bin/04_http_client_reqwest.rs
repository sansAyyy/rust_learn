use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct HttpBinGet {
    url: String,
    origin: String,
}

fn main() -> Result<()> {
    // 默认不发真实网络请求，避免离线环境运行失败。
    // 设置 RUN_HTTP_EXAMPLE=1 后会请求 https://httpbin.org/get。
    if std::env::var("RUN_HTTP_EXAMPLE").as_deref() != Ok("1") {
        println!("HTTP example is compiled but skipped.");
        println!("Set RUN_HTTP_EXAMPLE=1 to perform a real request.");
        return Ok(());
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("rust-learn-practical/0.1")
        .build()
        .context("build HTTP client")?;

    let response = client
        .get("https://httpbin.org/get")
        .send()
        .context("send HTTP request")?
        .error_for_status()
        .context("server returned error status")?
        .json::<HttpBinGet>()
        .context("parse JSON response")?;

    println!("url = {}", response.url);
    println!("origin = {}", response.origin);

    Ok(())
}
