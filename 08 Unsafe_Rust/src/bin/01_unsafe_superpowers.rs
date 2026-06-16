fn main() {
    // unsafe 允许使用五类“超能力”：
    // 1. 解引用裸指针。
    // 2. 调用 unsafe 函数或方法。
    // 3. 访问或修改可变静态变量。
    // 4. 实现 unsafe trait。
    // 5. 访问 union 字段。

    let mut number = 5;

    let raw_const = &number as *const i32;
    let raw_mut = &mut number as *mut i32;

    // 创建裸指针是安全的；解引用裸指针才需要 unsafe。
    unsafe {
        println!("raw_const points to {}", *raw_const);
        *raw_mut += 1;
        println!("raw_mut changed number to {}", *raw_mut);
    }

    println!("number = {number}");

    // unsafe 块内部也仍然遵守类型检查、生命周期检查等大部分 Rust 规则。
}
