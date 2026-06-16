//! 这是库 crate 的根模块。
//!
//! `src/lib.rs` 暴露出来的 public API 可以被：
//! - `src/main.rs`
//! - `src/bin/*.rs`
//! - `examples/*.rs`
//! - `tests/*.rs`
//! - 其他依赖这个库的项目
//! 使用。

pub mod config;
pub mod math;
pub mod models;

// pub use 可以重新导出常用类型，让外部调用路径更短。
pub use config::Config;
pub use math::add;
pub use models::User;

/// 返回当前库的简短说明。
///
/// # Examples
///
/// ```
/// let name = cargo_project_organization::crate_name();
/// assert_eq!(name, "cargo_project_organization");
/// ```
pub fn crate_name() -> &'static str {
    "cargo_project_organization"
}
