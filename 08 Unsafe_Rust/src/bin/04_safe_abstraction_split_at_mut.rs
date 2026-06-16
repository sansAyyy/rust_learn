fn split_at_mut_custom<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len, "mid must be within slice bounds");

    unsafe {
        // Safety:
        // - ptr 来自一个有效的 &mut [T]。
        // - mid <= len 已检查。
        // - 两个切片覆盖不重叠的内存区域：0..mid 和 mid..len。
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut values = [1, 2, 3, 4, 5];

    let (left, right) = split_at_mut_custom(&mut values, 2);
    left[0] = 10;
    right[0] = 30;

    println!("values = {:?}", values);

    // 这就是 unsafe 的理想使用方式：
    // 内部使用小范围 unsafe，外部暴露安全 API。
}
