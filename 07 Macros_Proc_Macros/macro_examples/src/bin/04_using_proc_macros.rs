use learning_macros::{log_call, make_answer, Describe};

#[derive(Describe)]
struct User {
    id: u64,
    name: String,
}

#[derive(Describe)]
enum Command {
    Start,
    Stop,
}

#[log_call]
fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    // derive 宏为类型生成 trait impl。
    let user = User {
        id: 1,
        name: String::from("Alice"),
    };
    println!("{}", user.describe());
    println!("user fields are still usable: {} {}", user.id, user.name);
    println!("{}", Command::Start.describe());
    println!("{}", Command::Stop.describe());

    // 函数式过程宏像普通宏一样调用，但由 proc-macro crate 生成 token。
    let answer = make_answer!();
    println!("answer from proc macro = {answer}");

    // 属性宏可以改写被标注的 item。
    println!("add result = {}", add(2, 3));
}
