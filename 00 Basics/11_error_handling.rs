use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic! 表示不可恢复错误，会终止当前线程。
    // panic!("something went terribly wrong");

    // Result<T, E> 表示可恢复错误：Ok(T) 或 Err(E)。
    match open_or_create_file("hello.txt") {
        Ok(_) => println!("opened or created hello.txt"),
        Err(error) => println!("file operation failed: {error}"),
    }

    match read_username_from_file("username.txt") {
        Ok(username) => println!("username = {username}"),
        Err(error) => println!("could not read username.txt: {error}"),
    }

    // unwrap 简洁但失败会 panic，适合示例或你确定不会失败的情况。
    let parsed: i32 = "42".parse().unwrap();
    println!("parsed = {parsed}");

    // expect 和 unwrap 类似，但能提供更清楚的错误信息。
    let parsed_again: i32 = "100"
        .parse()
        .expect("literal 100 should always parse as i32");
    println!("parsed_again = {parsed_again}");
}

fn open_or_create_file(path: &str) -> Result<File, io::Error> {
    let file_result = File::open(path);

    match file_result {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create(path),
            _ => Err(error),
        },
    }
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    // ? 会在 Err 时提前返回错误，在 Ok 时取出成功值。
    File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}
