mod user;

// user 模块是私有的，但 User 类型被重新导出为 public API。
pub use user::User;
