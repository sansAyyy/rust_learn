fn main() {
    let values = [10, 20, 30, 40];
    let ptr = values.as_ptr();

    unsafe {
        // add 是指针偏移。调用者必须保证偏移后的指针仍然有效。
        println!("first = {}", *ptr);
        println!("third = {}", *ptr.add(2));
    }

    let mut value = 100;
    let ptr_a = &mut value as *mut i32;
    let ptr_b = ptr_a;

    unsafe {
        // 裸指针可以有多个可变指针指向同一位置。
        // 这就是 unsafe 的危险之一：你必须自己保证不会产生未定义行为。
        *ptr_a += 1;
        *ptr_b += 1;
    }

    println!("value = {value}");

    // 不要解引用空指针、悬垂指针、越界指针或未对齐指针。
}
