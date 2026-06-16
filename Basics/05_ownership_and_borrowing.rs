fn main() {
    // 所有权规则：
    // 1. Rust 中每个值都有一个所有者。
    // 2. 同一时间只能有一个所有者。
    // 3. 所有者离开作用域时，值会被丢弃。

    let s1 = String::from("hello");
    let s2 = s1; // String 移动到 s2，s1 不再可用。
    println!("s2 = {s2}");
    // println!("{s1}"); // 编译错误：s1 的所有权已移动。

    // clone 会深拷贝堆上数据，两个变量都可用。
    let a = String::from("copy me");
    let b = a.clone();
    println!("a = {a}, b = {b}");

    // Copy 类型（如整数、bool、char、浮点数、只含 Copy 的元组）赋值时复制。
    let n1 = 10;
    let n2 = n1;
    println!("n1 = {n1}, n2 = {n2}");

    let text = String::from("ownership");
    takes_ownership(text);
    // println!("{text}"); // 编译错误：text 已被移动进函数。

    let number = 5;
    makes_copy(number);
    println!("number still usable = {number}");

    let word = String::from("borrow");
    let len = calculate_length(&word); // 借用，不取得所有权。
    println!("length of '{word}' = {len}");

    let mut message = String::from("hello");
    change(&mut message); // 可变借用允许被借用方修改值。
    println!("message = {message}");

    // 同一作用域内，可以有多个不可变引用，或一个可变引用，但不能同时拥有二者。
    let mut data = String::from("data");
    let r1 = &data;
    let r2 = &data;
    println!("immutable borrows: {r1}, {r2}");

    let r3 = &mut data;
    r3.push_str(" changed");
    println!("mutable borrow: {r3}");
}

fn takes_ownership(s: String) {
    println!("took ownership of {s}");
}

fn makes_copy(n: i32) {
    println!("copied number = {n}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", Rust");
}
