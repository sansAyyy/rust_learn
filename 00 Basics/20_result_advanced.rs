use std::fmt;
use std::num::ParseIntError;

fn main() {
    // Result<T, E> 表达“成功值 T 或错误 E”。
    let good = parse_positive_number("42");
    let bad = parse_positive_number("-7");
    let invalid = parse_positive_number("abc");
    println!("good = {:?}", good);
    println!("bad = {:?}", bad);
    println!("invalid = {:?}", invalid);

    // map：转换 Ok 内部值。
    let doubled = parse_positive_number("21").map(|n| n * 2);
    println!("doubled = {:?}", doubled);

    // map_err：转换 Err 内部值，常用于补充上下文或统一错误类型。
    let labeled = "not-a-number"
        .parse::<i32>()
        .map_err(|err| format!("parse failed: {err}"));
    println!("labeled error = {:?}", labeled);

    // and_then：连续执行可能失败的操作。
    let parsed_even = parse_positive_number("24").and_then(require_even);
    println!("parsed_even = {:?}", parsed_even);

    // unwrap_or_else：失败时计算默认值。
    let fallback = parse_positive_number("oops").unwrap_or_else(|err| {
        println!("using fallback because: {err}");
        0
    });
    println!("fallback = {fallback}");

    // ? 操作符：Err 时提前返回，Ok 时取出值。
    match load_config("localhost:8080") {
        Ok(config) => {
            println!("config = {:?}", config);
            println!("connect to {} on port {}", config.host, config.port);
        }
        Err(err) => println!("config error = {err}"),
    }

    match load_config("localhost:not-a-port") {
        Ok(config) => println!("config = {:?}", config),
        Err(err) => println!("config error = {err}"),
    }

    // Result 可以收集迭代器：只要一个元素是 Err，整体就是 Err。
    let texts = ["1", "2", "3"];
    let numbers: Result<Vec<i32>, ParseIntError> =
        texts.iter().map(|text| text.parse::<i32>()).collect();
    println!("collected numbers = {:?}", numbers);

    let texts_with_error = ["1", "x", "3"];
    let failed_numbers: Result<Vec<i32>, ParseIntError> =
        texts_with_error.iter().map(|text| text.parse::<i32>()).collect();
    println!("failed numbers = {:?}", failed_numbers);
}

#[derive(Debug)]
enum AppError {
    Empty,
    Parse(ParseIntError),
    NotPositive(i32),
    NotEven(i32),
    InvalidConfig(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Empty => write!(f, "input is empty"),
            AppError::Parse(err) => write!(f, "could not parse integer: {err}"),
            AppError::NotPositive(n) => write!(f, "{n} is not positive"),
            AppError::NotEven(n) => write!(f, "{n} is not even"),
            AppError::InvalidConfig(text) => write!(f, "invalid config: {text}"),
        }
    }
}

impl std::error::Error for AppError {}

// From 让 ? 可以自动把 ParseIntError 转换成 AppError。
impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError::Parse(value)
    }
}

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn parse_positive_number(input: &str) -> Result<i32, AppError> {
    if input.trim().is_empty() {
        return Err(AppError::Empty);
    }

    let number: i32 = input.parse()?; // ParseIntError 自动转 AppError。

    if number <= 0 {
        return Err(AppError::NotPositive(number));
    }

    Ok(number)
}

fn require_even(number: i32) -> Result<i32, AppError> {
    if number % 2 == 0 {
        Ok(number)
    } else {
        Err(AppError::NotEven(number))
    }
}

fn load_config(text: &str) -> Result<Config, AppError> {
    let (host, port_text) = text
        .split_once(':')
        .ok_or_else(|| AppError::InvalidConfig(String::from("expected host:port")))?;

    let port = port_text.parse::<u16>()?;

    Ok(Config {
        host: host.to_string(),
        port,
    })
}
