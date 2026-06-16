//! 测试与调试示例库。
//!
//! 文档测试会在 `cargo test` 时自动运行。
//!
//! ```
//! let total = testing_debugging::calculator::add(2, 3);
//! assert_eq!(total, 5);
//! ```

pub mod calculator;
pub mod parser;
pub mod store;

/// 判断字符串是否是回文。
///
/// ```
/// assert!(testing_debugging::is_palindrome("level"));
/// assert!(!testing_debugging::is_palindrome("rust"));
/// ```
pub fn is_palindrome(input: &str) -> bool {
    let normalized: String = input
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect();

    normalized.chars().eq(normalized.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_palindrome() {
        assert!(is_palindrome("Never odd or even"));
    }

    #[test]
    fn rejects_non_palindrome() {
        assert!(!is_palindrome("Rust"));
    }
}
