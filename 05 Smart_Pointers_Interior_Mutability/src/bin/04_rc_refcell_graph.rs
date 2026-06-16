use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &str) -> Rc<Node> {
        Rc::new(Node {
            name: name.to_string(),
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        parent.children.borrow_mut().push(child);
    }
}

fn main() {
    // Rc<RefCell<T>> 是单线程中“多所有者 + 可修改”的常见组合。
    // 适合树、图、UI 节点、测试替身等场景。
    let root = Node::new("root");
    let left = Node::new("left");
    let right = Node::new("right");

    Node::add_child(&root, Rc::clone(&left));
    Node::add_child(&root, Rc::clone(&right));

    println!("root strong count = {}", Rc::strong_count(&root));
    println!("left strong count = {}", Rc::strong_count(&left));

    for child in root.children.borrow().iter() {
        println!("child = {}", child.name);
    }

    let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));
    let owner_a = Rc::clone(&shared_list);
    let owner_b = Rc::clone(&shared_list);

    owner_a.borrow_mut().push(4);
    owner_b.borrow_mut().push(5);

    println!("shared list = {:?}", shared_list.borrow());

    // 注意：Rc<RefCell<T>> 不是线程安全的，跨线程用 Arc<Mutex<T>>。
}
