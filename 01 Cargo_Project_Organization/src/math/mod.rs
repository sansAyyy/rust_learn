// mod.rs 是模块目录布局的一种写法。
// `pub mod stats;` 会加载同目录下的 stats.rs。
pub mod stats;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub(crate) fn internal_double(value: i32) -> i32 {
    value * 2
}

pub fn double_for_public_api(value: i32) -> i32 {
    internal_double(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn crate_private_items_are_testable_inside_crate() {
        assert_eq!(internal_double(4), 8);
    }
}
