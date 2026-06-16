use std::fmt;

trait Describe {
    fn describe(&self) -> String;
}

struct LocalType {
    value: i32,
}

impl Describe for LocalType {
    fn describe(&self) -> String {
        format!("LocalType({})", self.value)
    }
}

// 孤儿规则 orphan rule：
// 只有当 trait 或类型至少有一个定义在当前 crate 中，才能写 impl。
// 这里 Display 是外部 trait，但 LocalType 是本地类型，所以允许。
impl fmt::Display for LocalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "local value: {}", self.value)
    }
}

// 下面不允许：Display 和 Vec 都不是本地定义。
// impl fmt::Display for Vec<i32> { ... }

trait CloneableWidget {
    fn name(&self) -> &str;

    // 返回 Self 的方法会让 trait 不适合作为 dyn Trait 调用。
    // 加上 `where Self: Sized` 后，该方法只给具体类型使用，trait object 仍可用。
    fn duplicate(&self) -> Self
    where
        Self: Sized;
}

#[derive(Clone)]
struct TextBox {
    name: String,
}

impl CloneableWidget for TextBox {
    fn name(&self) -> &str {
        &self.name
    }

    fn duplicate(&self) -> Self {
        self.clone()
    }
}

fn main() {
    let local = LocalType { value: 10 };
    println!("{}", local);
    println!("{}", local.describe());

    let textbox = TextBox {
        name: String::from("search"),
    };
    let copied = textbox.duplicate();
    println!("copied concrete widget = {}", copied.name());

    let object: Box<dyn CloneableWidget> = Box::new(textbox);
    println!("trait object widget = {}", object.name());
    // object.duplicate(); // 编译错误：duplicate 要求 Self: Sized，dyn Trait 大小未知。

    // 对象安全 object safety 简化理解：
    // - 方法不能在 trait object 上要求返回 Self，除非加 where Self: Sized。
    // - 方法不能有泛型参数，除非只给 Sized 类型使用。
    // - dyn Trait 适合“运行时多态”，但不是所有 trait 都能做成 trait object。
}
