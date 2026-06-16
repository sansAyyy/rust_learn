fn main() {
    let mut value = 10;

    // 引用允许访问值而不取得所有权。
    let read_ref = &value;
    println!("read_ref = {read_ref}");

    // 可变引用允许修改值。
    let write_ref = &mut value;
    *write_ref += 5; // * 解引用，访问引用指向的值。
    println!("value after mutation = {value}");

    let text = String::from("hello rust world");
    let first = first_word(&text);
    println!("first word = {first}");

    // 字符串切片 &str 引用字符串的一部分，不拥有数据。
    let hello = &text[0..5];
    let rust = &text[6..10];
    println!("slices: {hello}, {rust}");

    // 范围可以省略开头或结尾。
    let from_start = &text[..5];
    let to_end = &text[11..];
    let whole = &text[..];
    println!("from_start = {from_start}, to_end = {to_end}, whole = {whole}");

    // 数组切片 &[T] 引用数组的一段。
    let numbers = [1, 2, 3, 4, 5];
    let middle = &numbers[1..4];
    println!("middle slice = {:?}", middle);

    // 切片边界必须落在有效边界上。
    // 对 UTF-8 字符串而言，边界必须是字符边界。
    let chinese = "你好 Rust";
    println!("first Chinese char = {}", &chinese[0..3]);
}

// 参数写 &str 比 &String 更通用：String 和字符串字面量都能传入。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    s
}
