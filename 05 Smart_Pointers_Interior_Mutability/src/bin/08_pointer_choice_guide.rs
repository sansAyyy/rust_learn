fn main() {
    println!("智能指针选择速查：");
    println!("Box<T>            单一所有者，堆分配，递归类型，trait object");
    println!("Rc<T>             单线程，多所有者，只读共享");
    println!("Rc<RefCell<T>>    单线程，多所有者，可修改共享");
    println!("Arc<T>            多线程，多所有者，只读共享");
    println!("Arc<Mutex<T>>     多线程，多所有者，可修改共享，互斥访问");
    println!("Arc<RwLock<T>>    多线程，多读单写");
    println!("Cell<T>           单线程，Copy 值内部可变性");
    println!("RefCell<T>        单线程，运行时借用检查");
    println!("Weak<T>           非拥有引用，避免 Rc/Arc 引用循环");
    println!("Cow<'a, T>        借用优先，需要修改时克隆成拥有值");

    println!();
    println!("经验规则：");
    println!("1. 能用普通所有权和借用，就先不用智能指针。");
    println!("2. 需要递归类型或 trait object 所有权时，用 Box。");
    println!("3. 需要多个所有者时，单线程 Rc，多线程 Arc。");
    println!("4. 需要共享同时修改时，单线程 RefCell，多线程 Mutex/RwLock。");
    println!("5. 看到 Rc<RefCell<T>> 或 Arc<Mutex<T>>，就要认真思考生命周期和锁范围。");
}
