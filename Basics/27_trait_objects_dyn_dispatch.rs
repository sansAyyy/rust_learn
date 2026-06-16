trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

struct Checkbox {
    checked: bool,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button: [{}]", self.label);
    }
}

impl Draw for Checkbox {
    fn draw(&self) {
        let mark = if self.checked { "x" } else { " " };
        println!("draw checkbox: [{mark}]");
    }
}

struct Screen {
    // Vec<Box<dyn Draw>> 可以存放不同具体类型，只要它们都实现 Draw。
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                label: String::from("OK"),
            }),
            Box::new(Checkbox { checked: true }),
        ],
    };
    screen.run();

    // &dyn Draw 是借用形式的 trait object，不取得所有权。
    let button = Button {
        label: String::from("Borrowed"),
    };
    draw_one(&button);

    // Box<dyn Draw> 是拥有形式，适合放入集合或返回。
    let boxed = make_component(true);
    boxed.draw();
}

fn draw_one(component: &dyn Draw) {
    component.draw();
}

fn make_component(flag: bool) -> Box<dyn Draw> {
    if flag {
        Box::new(Checkbox { checked: false })
    } else {
        Box::new(Button {
            label: String::from("Fallback"),
        })
    }
}

// 静态分发：T: Draw，编译期知道具体类型。
// 动态分发：dyn Draw，运行时通过 vtable 调用方法。
// 当需要同一个集合放多种类型时，dyn Trait 很常见。
