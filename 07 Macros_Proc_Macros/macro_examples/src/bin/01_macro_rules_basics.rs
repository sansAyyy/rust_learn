macro_rules! say_hello {
    () => {
        println!("hello from a macro");
    };
    ($name:expr) => {
        println!("hello, {}", $name);
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("called function `{}`", stringify!($func_name));
        }
    };
}

create_function!(generated_function);

fn main() {
    // macro_rules! 通过模式匹配 token，然后展开成 Rust 代码。
    say_hello!();
    say_hello!("Rust");

    generated_function();

    // 常见片段说明：
    // expr  表达式
    // ident 标识符
    // ty    类型
    // stmt  语句
    // block 代码块
    // pat   模式
}
