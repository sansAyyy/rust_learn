// 一个包 package 通常由 Cargo.toml 描述，可以包含一个或多个 crate。
// 一个 crate 是 Rust 的编译单元：可以是二进制 crate，也可以是库 crate。
// 本文件是一个独立二进制 crate；下面用内联 mod 演示模块系统。

mod front_of_house {
    // pub 让模块、函数、类型或字段对父模块可见。
    pub mod hosting {
        pub fn add_to_waitlist(name: &str) {
            println!("adding {name} to waitlist");
        }

        fn seat_at_table() {
            println!("seat at table");
        }

        pub fn demo_private_helper() {
            // 同一模块内可以调用私有函数。
            seat_at_table();
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // 字段也需要单独 pub。
        seasonal_fruit: String, // 私有字段只能在模块内部访问。
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn describe(&self) {
            println!("{} toast with {}", self.toast, self.seasonal_fruit);
        }
    }
}

// use 把路径引入当前作用域，减少重复书写。
use crate::front_of_house::hosting;

fn main() {
    // 绝对路径从 crate 根开始。
    crate::front_of_house::hosting::add_to_waitlist("Alice");

    // 相对路径从当前模块开始。
    hosting::add_to_waitlist("Bob");
    hosting::demo_private_helper();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    meal.describe();
    // meal.seasonal_fruit = String::from("blueberries"); // 编译错误：字段私有。

    // 真实项目中常见布局：
    // src/main.rs      二进制入口
    // src/lib.rs       库 crate 入口
    // src/foo.rs       foo 模块
    // src/foo/bar.rs   foo::bar 子模块
}
