fn main() {
    let name = Some("Alice");
    let missing_name: Option<&str> = None;

    // map：只在 Some 时转换内部值。
    let greeting = name.map(|n| format!("Hello, {n}"));
    println!("greeting = {:?}", greeting);

    // unwrap_or / unwrap_or_else：提供默认值。
    println!("name = {}", name.unwrap_or("stranger"));
    println!(
        "missing = {}",
        missing_name.unwrap_or_else(|| "computed stranger")
    );

    // and_then：返回 Option 的转换，避免 Option<Option<T>>。
    let port_text = Some("8080");
    let port = port_text.and_then(parse_port);
    println!("port = {:?}", port);

    // filter：Some 值满足条件则保留，否则变为 None。
    let valid_port = port.filter(|p| *p > 0 && *p < 65535);
    println!("valid_port = {:?}", valid_port);

    // or / or_else：提供备选 Option。
    let configured = None;
    let from_env = Some("env-value");
    println!("fallback = {:?}", configured.or(from_env));

    // as_ref：把 Option<T> 转成 Option<&T>，避免移动所有权。
    let maybe_string = Some(String::from("owned text"));
    let length = maybe_string.as_ref().map(|s| s.len());
    println!("length = {:?}, original = {:?}", length, maybe_string);

    // ok_or / ok_or_else：Option 转 Result，常用于把“缺值”变成错误。
    let user_id = find_user_id("alice").ok_or_else(|| String::from("user not found"));
    println!("user_id result = {:?}", user_id);

    // if let 适合只处理 Some 的场景。
    if let Some(id) = find_user_id("bob") {
        println!("bob id = {id}");
    }

    // let-else 适合提前返回或提前退出。
    print_user("carol");
    print_user("nobody");

    // ? 也可以用于返回 Option 的函数。
    println!("first char = {:?}", first_char_of_user("alice"));
    println!("first char missing = {:?}", first_char_of_user("nobody"));
}

fn parse_port(text: &str) -> Option<u16> {
    text.parse::<u16>().ok()
}

fn find_user_id(name: &str) -> Option<u64> {
    match name {
        "alice" => Some(1),
        "bob" => Some(2),
        "carol" => Some(3),
        _ => None,
    }
}

fn print_user(name: &str) {
    let Some(id) = find_user_id(name) else {
        println!("user '{name}' does not exist");
        return;
    };

    println!("user '{name}' has id {id}");
}

fn first_char_of_user(name: &str) -> Option<char> {
    let id = find_user_id(name)?;
    let display = format!("user-{id}");
    display.chars().next()
}
