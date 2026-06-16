use std::env;
use std::time::{Duration, Instant};

fn main() {
    // println! / print!：格式化输出。
    println!("Hello, {}!", "Rust");

    // format! 返回 String，不直接输出。
    let message = format!("{} + {} = {}", 2, 3, 2 + 3);
    println!("{message}");

    // dbg! 打印表达式和位置到 stderr，并返回表达式的所有权。
    let value = dbg!(10 * 2);
    println!("value from dbg = {value}");

    // vec! 快速创建 Vec。
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    println!("doubled = {:?}", doubled);

    // assert! / assert_eq! / assert_ne! 常用于测试和运行时校验。
    assert!(doubled.contains(&4));
    assert_eq!(doubled[0], 2);
    assert_ne!(doubled.len(), 0);

    // Option 常用组合子。
    let maybe_name = Some("Alice");
    let greeting = maybe_name
        .map(|name| format!("Hi, {name}"))
        .unwrap_or_else(|| String::from("Hi, stranger"));
    println!("{greeting}");

    // Result 常用组合子。
    let parsed = "42"
        .parse::<i32>()
        .map(|n| n + 1)
        .unwrap_or_else(|_| 0);
    println!("parsed + 1 = {parsed}");

    // Iterator 常见操作：filter、map、sum。
    let sum_of_even_squares: i32 = (1..=10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .sum();
    println!("sum of even squares = {sum_of_even_squares}");

    // std::env 读取命令行参数。
    let args: Vec<String> = env::args().collect();
    println!("program args = {:?}", args);

    // std::time 测量耗时。
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    println!("elapsed = {:?}", start.elapsed());

    // include_str! 在编译期把文件内容嵌入二进制。
    // let cargo_toml = include_str!("Cargo.toml");
}
