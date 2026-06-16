fn main() {
    let number = 7;

    // if 条件必须是 bool，不会自动把整数当真假值。
    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number equals 5");
    } else {
        println!("number is greater than 5");
    }

    // if 是表达式，所以可以赋值。每个分支返回类型必须一致。
    let label = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {label}");

    // loop 创建无限循环，可以用 break 返回值。
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 10;
        }
    };
    println!("loop result = {result}");

    // while 在条件为 true 时循环。
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("go!");

    // for 常用于遍历集合，比手写索引更安全。
    let items = ["red", "green", "blue"];
    for item in items {
        println!("color = {item}");
    }

    // 范围语法：1..4 不包含 4，1..=4 包含 4。
    for n in 1..=4 {
        println!("n = {n}");
    }

    // match 用于多分支模式匹配，必须覆盖所有可能情况。
    let command = "start";
    match command {
        "start" => println!("starting"),
        "stop" => println!("stopping"),
        other => println!("unknown command: {other}"),
    }
}
