#[derive(Debug)]
struct Highlight<'a> {
    source: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Highlight<'a> {
    fn text(&self) -> &'a str {
        &self.source[self.start..self.end]
    }

    fn contains(&self, keyword: &str) -> bool {
        self.text().contains(keyword)
    }
}

#[derive(Debug)]
struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn first_token(&self) -> Option<&'a str> {
        self.input.split_whitespace().next()
    }
}

#[derive(Debug)]
struct OwnedNote {
    title: String,
    body: String,
}

fn main() {
    let document = String::from("Rust ownership keeps memory safe");

    // 结构体可以持有引用，但必须标注生命周期，说明引用不能比源数据活得更久。
    let highlight = Highlight {
        source: &document,
        start: 5,
        end: 14,
    };
    println!("highlight = {:?}", highlight);
    println!("highlight text = {}", highlight.text());
    println!("contains ownership? {}", highlight.contains("ownership"));

    let parser = Parser::new(&document);
    println!("first token = {:?}", parser.first_token());

    // 如果结构体需要独立存在，通常让它拥有数据，而不是保存引用。
    let note = OwnedNote {
        title: String::from("Owned data"),
        body: document.clone(),
    };
    println!("owned note = {:?}", note);
    println!("note title = {}, body length = {}", note.title, note.body.len());

    // 下面这种写法不能通过编译：temp 离开作用域后，dangling 会悬垂。
    // let dangling;
    // {
    //     let temp = String::from("temporary");
    //     dangling = Highlight { source: &temp, start: 0, end: 4 };
    // }
    // println!("{:?}", dangling);
}
