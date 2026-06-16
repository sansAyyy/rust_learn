use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &str) -> Rc<Node> {
        Rc::new(Node {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
}

fn main() {
    // Rc 循环引用会导致内存无法释放。
    // 父 -> 子 用 Rc 表示拥有；子 -> 父 用 Weak 表示不拥有。
    let branch = Node::new("branch");
    let leaf = Node::new("leaf");

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    branch.children.borrow_mut().push(Rc::clone(&leaf));

    println!(
        "leaf parent = {:?}",
        leaf.parent.borrow().upgrade().map(|node| node.name.clone())
    );
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    drop(branch);

    // branch 被释放后，Weak::upgrade 返回 None。
    println!(
        "leaf parent after branch dropped = {:?}",
        leaf.parent.borrow().upgrade().map(|node| node.name.clone())
    );
}
