use std::fmt;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn prepend(self, value: i32) -> List {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Cons(_, next) => 1 + next.len(),
            List::Nil => 0,
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons(value, next) => write!(f, "{value} -> {:?}", next),
            List::Nil => write!(f, "Nil"),
        }
    }
}

trait Draw {
    fn draw(&self);
}

struct Button;
struct Label;

impl Draw for Button {
    fn draw(&self) {
        println!("draw button");
    }
}

impl Draw for Label {
    fn draw(&self) {
        println!("draw label");
    }
}

fn main() {
    // Box<T> 把值放在堆上，栈上只保存一个固定大小的指针。
    let number = Box::new(42);
    println!("boxed number = {number}");

    // 递归类型必须通过 Box/Rc 等指针打断无限大小。
    let list = List::Nil.prepend(3).prepend(2).prepend(1);
    println!("list = {:?}, len = {}", list, list.len());

    // Box<dyn Trait> 可以拥有一个 trait object。
    let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Label)];
    for widget in widgets {
        widget.draw();
    }
}
