use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    features: FeatureConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct FeatureConfig {
    enable_cache: bool,
    worker_count: usize,
}

fn main() -> Result<()> {
    // 实战中常见配置来源优先级：
    // 默认值 < 配置文件 < 环境变量 < 命令行参数。
    let raw = r#"
        [server]
        host = "127.0.0.1"
        port = 8080

        [features]
        enable_cache = true
        worker_count = 4
    "#;

    let mut config: AppConfig = toml::from_str(raw).context("parse embedded TOML config")?;

    if let Ok(port) = std::env::var("APP_PORT") {
        config.server.port = port
            .parse::<u16>()
            .with_context(|| format!("parse APP_PORT={port}"))?;
    }

    println!("config = {:?}", config);
    println!(
        "listen on {}:{}, cache={}, workers={}",
        config.server.host,
        config.server.port,
        config.features.enable_cache,
        config.features.worker_count
    );

    Ok(())
}
