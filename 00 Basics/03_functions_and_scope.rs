fn main() {
    greet("Rust");

    let sum = add(3, 4);
    println!("3 + 4 = {sum}");

    // Rust 是表达式导向语言。代码块可以产生值。
    let y = {
        let x = 3;
        x + 1 // 没有分号，作为代码块返回值。
    };
    println!("y = {y}");

    // 有分号的是语句，语句不返回有意义的值。
    let unit = {
        let _x = 3;
    };
    println!("unit from statement block = {:?}", unit);

    let value = multiply_then_add(2, 3, 4);
    println!("2 * 3 + 4 = {value}");

    // 作用域控制变量生命周期。
    let outer = "outer";
    {
        let inner = "inner";
        println!("{outer}, {inner}");
    }
    // println!("{inner}"); // 编译错误：inner 只在内部代码块有效。
}

// 函数使用 fn 定义，参数必须写类型。
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// 返回值类型写在 -> 后。
// 函数体最后一个表达式会成为返回值。
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply_then_add(a: i32, b: i32, c: i32) -> i32 {
    let product = a * b;
    product + c
}
