// 生命周期描述引用保持有效的范围，帮助编译器防止悬垂引用。

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("longest = {result}");

    // result 的生命周期不能超过它引用的数据。
    let outer_result;
    {
        let inner = String::from("long string is long");
        outer_result = longest(string1.as_str(), inner.as_str());
        println!("inner scope result = {outer_result}");
    }
    // println!("{outer_result}"); // 编译错误：可能引用已释放的 inner。

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("could not find a period");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("excerpt = {}", excerpt.announce_and_return_part("note"));

    let static_text: &'static str = "I live for the entire program";
    println!("{static_text}");
}

// 'a 表示返回引用的生命周期与两个输入引用中较短的那个一致。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 生命周期省略规则通常能让方法签名更简洁。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {announcement}");
        self.part
    }
}
