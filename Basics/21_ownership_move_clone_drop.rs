#[derive(Debug, Clone)]
struct Document {
    title: String,
    words: Vec<String>,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // String、Vec、Box 等拥有堆数据的类型，赋值或传参默认是 move。
    let doc1 = Document {
        title: String::from("Rust Notes"),
        words: vec![String::from("ownership"), String::from("borrowing")],
    };

    let doc2 = doc1;
    println!("doc2 = {:?}", doc2);
    // println!("{:?}", doc1); // 编译错误：doc1 已经移动到 doc2。

    // clone 会显式复制堆数据，成本由你主动承担。
    let doc3 = doc2.clone();
    println!("doc2 still usable = {:?}", doc2);
    println!("doc3 cloned = {:?}", doc3);

    // Copy 类型赋值后两个变量都可用。
    let p1 = Point { x: 3, y: 4 };
    let p2 = p1;
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
    println!("point sum = {}", p1.x + p1.y + p2.x + p2.y);

    // 传参也遵守所有权。
    print_title_by_borrow(&doc2);
    println!("doc2 after borrow = {:?}", doc2);

    let consumed_title = take_title(doc3);
    println!("consumed title = {consumed_title}");
    // println!("{:?}", doc3); // 编译错误：doc3 已被移动进 take_title。

    // 部分移动：移动结构体的某个字段后，整个结构体不能再完整使用。
    let person = (String::from("Alice"), 30);
    let name = person.0;
    println!("moved name = {name}, age still copyable = {}", person.1);
    // println!("{:?}", person); // 编译错误：person.0 已被移动。

    // Drop 在变量离开作用域时自动运行，用于释放资源。
    {
        let _guard = ResourceGuard::new("temporary file handle");
        println!("inside inner scope");
    }
    println!("resource was dropped before this line");
}

fn print_title_by_borrow(document: &Document) {
    println!(
        "borrowed title = {}, word count = {}",
        document.title,
        document.words.len()
    );
}

fn take_title(document: Document) -> String {
    document.title
}

struct ResourceGuard {
    name: String,
}

impl ResourceGuard {
    fn new(name: &str) -> Self {
        println!("acquire resource: {name}");
        Self {
            name: name.to_string(),
        }
    }
}

impl Drop for ResourceGuard {
    fn drop(&mut self) {
        println!("drop resource: {}", self.name);
    }
}
