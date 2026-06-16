use std::collections::HashMap;

fn main() {
    // Vec<T>：连续存储、可增长的动态数组。
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("numbers = {:?}", numbers);

    let second = &numbers[1]; // 越界会 panic。
    println!("second by index = {second}");

    match numbers.get(10) {
        Some(value) => println!("found value = {value}"),
        None => println!("index 10 is out of range"),
    }

    for number in &mut numbers {
        *number += 1;
    }
    println!("numbers after mutation = {:?}", numbers);

    // String：UTF-8 编码、可增长的字符串。
    let mut text = String::from("hello");
    text.push(',');
    text.push_str(" Rust");
    println!("text = {text}");

    // 字符串索引不直接支持，因为 UTF-8 字符长度不固定。
    for ch in text.chars() {
        print!("{ch} ");
    }
    println!();

    // HashMap<K, V>：键值映射。
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores = {:?}", scores);

    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{team} score = {score}");

    // entry 常用于“没有就插入”或基于旧值更新。
    scores.entry(String::from("Blue")).or_insert(0);
    scores.entry(String::from("Green")).or_insert(30);

    let words = "hello world wonderful world";
    let mut counts = HashMap::new();
    for word in words.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word counts = {:?}", counts);
}
