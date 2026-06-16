fn main() {
    // Rust 变量默认不可变。不可变让代码更容易推理，也避免意外修改。
    let x = 5;
    println!("x = {x}");

    // x = 6; // 编译错误：不能给不可变变量重新赋值。

    // 使用 mut 创建可变变量。
    let mut score = 10;
    println!("score before = {score}");
    score += 5;
    println!("score after = {score}");

    // 常量使用 const，必须标注类型，且只能绑定到编译期可确定的表达式。
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");

    // shadowing：用 let 重新声明同名变量。
    // 这不是修改原变量，而是创建了一个新的绑定。
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces length = {spaces}");

    // shadowing 可以改变类型；mut 重新赋值不能改变类型。
    let answer = "42";
    let answer: i32 = answer.parse().expect("42 should be a number");
    println!("answer + 1 = {}", answer + 1);

    // 作用域结束后，变量不再可用。
    {
        let inner = "only visible in this block";
        println!("{inner}");
    }
    // println!("{inner}"); // 编译错误：inner 已离开作用域。
}
