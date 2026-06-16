unsafe fn get_unchecked(values: &[i32], index: usize) -> i32 {
    // Safety: 调用者必须保证 index < values.len()。
    unsafe { *values.as_ptr().add(index) }
}

fn safe_get(values: &[i32], index: usize) -> Option<i32> {
    if index < values.len() {
        // 这里由 safe_get 检查了不变量，所以可以安全封装 unsafe 调用。
        Some(unsafe { get_unchecked(values, index) })
    } else {
        None
    }
}

fn main() {
    let values = [1, 2, 3];

    println!("safe_get(1) = {:?}", safe_get(&values, 1));
    println!("safe_get(99) = {:?}", safe_get(&values, 99));

    unsafe {
        println!("direct unsafe get = {}", get_unchecked(&values, 2));
    }

    // unsafe 函数的文档必须说明调用者需要保证什么。
}
