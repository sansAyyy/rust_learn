use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

fn main() {
    // 应用程序入口常见写法：
    // main 负责把错误展示给用户并设置退出码，真正逻辑放在 run()。
    if let Err(error) = run() {
        eprintln!("error: {error}");

        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("caused by: {cause}");
            source = cause.source();
        }

        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> 常用于应用层：不同错误可以统一向上传播。
    // 库层更推荐具体错误类型，应用层更看重快速组合和清晰输出。
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("app.conf"));

    let config = read_config(&config_path)?;
    println!("loaded config from {config_path}: {:?}", config);
    println!("server will listen on {}:{}", config.host, config.port);

    Ok(())
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    // 这里故意让 app.conf 不存在时使用默认配置，演示“有些错误可以恢复”。
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            String::from("host=127.0.0.1\nport=8080")
        }
        Err(error) => return Err(Box::new(error)),
    };

    parse_config(&content).map_err(Into::into)
}

fn parse_config(content: &str) -> Result<Config, String> {
    let mut host = None;
    let mut port = None;

    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match key.trim() {
            "host" => host = Some(value.trim().to_string()),
            "port" => {
                let parsed = value
                    .trim()
                    .parse::<u16>()
                    .map_err(|error| format!("invalid port: {error}"))?;
                port = Some(parsed);
            }
            _ => {}
        }
    }

    Ok(Config {
        host: host.ok_or_else(|| String::from("missing host"))?,
        port: port.ok_or_else(|| String::from("missing port"))?,
    })
}
