unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn double_for_c(input: i32) -> i32 {
    input * 2
}

fn main() {
    // FFI：Foreign Function Interface，用于和 C ABI 交互。
    unsafe {
        println!("abs(-3) from C = {}", abs(-3));
    }

    // extern "C" fn 可以导出给 C 调用。
    println!("double_for_c(21) = {}", double_for_c(21));

    // FFI 真实项目中要格外注意：
    // - 字符串编码和所有权。
    // - 指针是否为空。
    // - 谁负责释放内存。
    // - ABI 和结构体布局。
}
