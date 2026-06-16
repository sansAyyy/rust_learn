#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message = {:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:?}, loopback = {:?}", home, loopback);

    match &home {
        IpAddr::V4(a, b, c, d) => println!("IPv4 parts = {a}.{b}.{c}.{d}"),
        IpAddr::V6(address) => println!("IPv6 address = {address}"),
    }

    match &loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4 parts = {a}.{b}.{c}.{d}"),
        IpAddr::V6(address) => println!("IPv6 address = {address}"),
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: 4 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in &messages {
        message.call();

        // match 会检查所有分支是否覆盖完整。
        match message {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move to ({x}, {y})"),
            Message::Write(text) => println!("write: {text}"),
            Message::ChangeColor(r, g, b) => println!("rgb = ({r}, {g}, {b})"),
        }
    }

    // Option<T> 表示“有值或无值”，避免空指针问题。
    let some_number = Some(5);
    let absent_number: Option<i32> = None;
    println!("some + 1 = {:?}", plus_one(some_number));
    println!("none + 1 = {:?}", plus_one(absent_number));

    // if let 用于只关心一种模式的场景。
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is configured as {max}");
    }

    // matches! 宏返回 bool，适合简洁判断模式。
    let status = Message::Quit;
    println!("is quit? {}", matches!(status, Message::Quit));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
