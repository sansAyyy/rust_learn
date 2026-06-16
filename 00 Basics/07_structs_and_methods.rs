#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(u8, u8, u8); // 元组结构体：字段没有名字，按位置访问。

struct AlwaysEqual; // 类单元结构体：没有字段。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块用于给类型实现方法和关联函数。
impl Rectangle {
    // 方法的第一个参数通常是 &self，表示借用当前实例。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 关联函数没有 self，常用于构造器。
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("new-alice@example.com");
    println!("user = {:?}", user);
    println!(
        "{} is active? {}, sign-ins = {}",
        user.username, user.active, user.sign_in_count
    );

    // 结构体更新语法：复用已有实例的剩余字段。
    let user2 = User {
        username: String::from("bob"),
        ..user
    };
    println!("user2 = {:?}", user2);
    // println!("{:?}", user); // 部分字段被移动后，user 不再完整可用。

    let black = Color(0, 0, 0);
    println!("black rgb = ({}, {}, {})", black.0, black.1, black.2);

    let _marker = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let square = Rectangle::square(20);

    println!("rect1 = {:?}, area = {}", rect1, rect1.area());
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("square = {:?}, area = {}", square, square.area());
}
