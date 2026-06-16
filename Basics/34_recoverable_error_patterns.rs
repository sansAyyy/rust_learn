use std::collections::HashMap;

fn main() {
    // 工程里常见错误处理模式：
    // 1. 立即返回错误。
    // 2. 提供默认值。
    // 3. 降级处理。
    // 4. 累积多个错误后统一返回。

    let settings = HashMap::from([
        ("host", "localhost"),
        ("port", "8080"),
        ("workers", "not-a-number"),
    ]);

    match build_server_config(&settings) {
        Ok(config) => {
            println!("config = {:?}", config);
            println!(
                "listen on {}:{} with {} workers",
                config.host, config.port, config.workers
            );
        }
        Err(errors) => {
            println!("found {} config error(s):", errors.len());
            for error in errors {
                println!("- {error}");
            }
        }
    }

    let timeout = parse_optional_u64(None, 30);
    println!("timeout default = {timeout}");

    let retry_count = parse_optional_u64(Some("bad"), 3);
    println!("retry_count fallback = {retry_count}");
}

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

fn build_server_config(settings: &HashMap<&str, &str>) -> Result<ServerConfig, Vec<String>> {
    let mut errors = Vec::new();

    let host = match settings.get("host").copied() {
        Some(value) if !value.trim().is_empty() => value.to_string(),
        _ => {
            errors.push(String::from("host is required"));
            String::new()
        }
    };

    let port = match settings.get("port").copied() {
        Some(value) => match value.parse::<u16>() {
            Ok(port) => port,
            Err(error) => {
                errors.push(format!("port is invalid: {error}"));
                0
            }
        },
        None => {
            errors.push(String::from("port is required"));
            0
        }
    };

    let workers = match settings.get("workers").copied() {
        Some(value) => match value.parse::<usize>() {
            Ok(workers) if workers > 0 => workers,
            Ok(_) => {
                errors.push(String::from("workers must be greater than zero"));
                0
            }
            Err(error) => {
                errors.push(format!("workers is invalid: {error}"));
                0
            }
        },
        None => 4, // 合理默认值。
    };

    if errors.is_empty() {
        Ok(ServerConfig {
            host,
            port,
            workers,
        })
    } else {
        Err(errors)
    }
}

fn parse_optional_u64(input: Option<&str>, default: u64) -> u64 {
    input
        .and_then(|text| text.parse::<u64>().ok())
        .unwrap_or(default)
}
