use std::rc::Rc;

#[derive(Debug)]
struct Document {
    title: String,
}

fn main() {
    // Rc<T> 是引用计数指针，允许单线程中多个所有者共享同一份数据。
    // Rc<T> 只能提供不可变共享；如果要修改，通常搭配 RefCell<T>。
    let document = Rc::new(Document {
        title: String::from("Rust Notes"),
    });

    println!("initial strong count = {}", Rc::strong_count(&document));

    let owner_a = Rc::clone(&document);
    let owner_b = Rc::clone(&document);

    println!(
        "after clones strong count = {}",
        Rc::strong_count(&document)
    );
    println!("owner_a title = {}", owner_a.title);
    println!("owner_b title = {}", owner_b.title);

    drop(owner_a);
    println!("after dropping owner_a = {}", Rc::strong_count(&document));

    let tags = Rc::new(vec![String::from("rust"), String::from("ownership")]);
    let article = Article {
        title: String::from("Rc example"),
        tags: Rc::clone(&tags),
    };
    let note = Note {
        text: String::from("shared tags"),
        tags: Rc::clone(&tags),
    };

    println!("article = {:?}", article);
    println!("note = {:?}", note);
    println!("tags strong count = {}", Rc::strong_count(&tags));
    println!(
        "article title = {}, first tag = {}",
        article.title, article.tags[0]
    );
    println!("note text = {}, tag count = {}", note.text, note.tags.len());
}

#[derive(Debug)]
struct Article {
    title: String,
    tags: Rc<Vec<String>>,
}

#[derive(Debug)]
struct Note {
    text: String,
    tags: Rc<Vec<String>>,
}
