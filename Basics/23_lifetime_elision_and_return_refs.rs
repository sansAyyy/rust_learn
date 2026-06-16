fn main() {
    let text = String::from("Rust language");

    // 返回引用时，返回值必须来自某个输入引用，不能引用函数内部临时值。
    let first = first_word(&text);
    println!("first word = {first}");

    let left = "short";
    let right = "much longer";
    let longer = longest(left, right);
    println!("longer = {longer}");

    let article = Article {
        title: String::from("Ownership"),
        body: String::from("Rust uses ownership to manage memory."),
    };

    let title = article.title();
    println!("title = {title}");

    let summary = article.summary();
    println!("summary = {summary}");

    // 当返回值需要新建内容时，返回 String，而不是 &str。
    let label = make_label("chapter", 23);
    println!("owned label = {label}");
}

// 生命周期省略规则：
// 只有一个输入引用时，输出引用默认与这个输入引用生命周期相同。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// 多个输入引用且返回引用时，编译器不知道返回值来自哪个参数，需要显式生命周期。
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

struct Article {
    title: String,
    body: String,
}

impl Article {
    // 方法中有 &self 时，输出引用默认与 self 的生命周期相关。
    fn title(&self) -> &str {
        &self.title
    }

    fn summary(&self) -> &str {
        let first_sentence = self.body.split('.').next().unwrap_or(&self.body);
        first_sentence
    }
}

fn make_label(prefix: &str, number: u32) -> String {
    format!("{prefix}-{number}")
}

// 这个函数无法编译，因为返回了指向局部 String 的引用。
// fn bad_reference() -> &str {
//     let value = String::from("temporary");
//     &value
// }
