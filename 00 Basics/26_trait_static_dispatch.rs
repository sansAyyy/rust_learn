use std::fmt::Display;

trait Render {
    fn render(&self) -> String;

    // 默认方法可以复用 trait 中的必需方法。
    fn render_with_border(&self) -> String {
        format!("| {} |", self.render())
    }
}

struct Button {
    label: String,
}

struct Label {
    text: String,
}

impl Render for Button {
    fn render(&self) -> String {
        format!("[ {} ]", self.label)
    }
}

impl Render for Label {
    fn render(&self) -> String {
        self.text.clone()
    }
}

fn main() {
    let button = Button {
        label: String::from("Save"),
    };
    let label = Label {
        text: String::from("Status: ready"),
    };

    // 泛型 + trait bound 使用静态分发。
    // 编译器会为具体类型生成专门代码，通常性能好，但二进制可能更大。
    print_rendered(&button);
    print_rendered(&label);

    // impl Trait 也可以作为参数类型，适合简单场景。
    print_inline(&button);

    // 多个 trait bound 可以用 + 组合。
    print_display_and_debug(42);

    // where 子句让复杂约束更清晰。
    compare_and_print("abc", "xyz");

    // 返回 impl Trait 表示“返回某个实现了该 trait 的具体类型”。
    let component = make_button("Cancel");
    println!("made component = {}", component.render());
}

fn print_rendered<T: Render>(item: &T) {
    println!("rendered = {}", item.render_with_border());
}

fn print_inline(item: &impl Render) {
    println!("inline = {}", item.render());
}

fn print_display_and_debug<T>(value: T)
where
    T: Display + std::fmt::Debug,
{
    println!("display = {value}, debug = {:?}", value);
}

fn compare_and_print<T, U>(left: T, right: U)
where
    T: Display + PartialOrd<U>,
    U: Display,
{
    if left < right {
        println!("{left} is smaller than {right}");
    } else {
        println!("{left} is not smaller than {right}");
    }
}

fn make_button(label: &str) -> impl Render {
    Button {
        label: label.to_string(),
    }
}
