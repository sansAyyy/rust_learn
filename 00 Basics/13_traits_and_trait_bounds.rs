// trait 定义一组共享行为，类似“能力协议”。
trait Summary {
    fn summarize_author(&self) -> String;

    // trait 可以提供默认实现。
    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust makes systems programming friendlier"),
        location: String::from("Internet"),
        author: String::from("Ferris"),
        content: String::from("Rust balances performance and safety."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Learning traits today!"),
        reply: false,
        retweet: false,
    };

    println!("article summary: {}", article.summarize());
    println!("tweet summary: {}", tweet.summarize());
    println!("article content preview: {}", article.content);
    println!(
        "tweet content: {}, reply? {}, retweet? {}",
        tweet.content, tweet.reply, tweet.retweet
    );

    notify(&article);
    notify_with_generic(&tweet);
    notify_two_bounds(&article);

    let pair = Pair { x: 3, y: 7 };
    pair.cmp_display();
}

// impl Trait 适合简单参数。
fn notify(item: &impl Summary) {
    println!("breaking news: {}", item.summarize());
}

// 泛型 trait bound 适合复杂签名。
fn notify_with_generic<T: Summary>(item: &T) {
    println!("generic notify: {}", item.summarize());
}

// where 子句让多个约束更易读。
fn notify_two_bounds<T>(item: &T)
where
    T: Summary,
{
    println!("where notify: {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

// 只有当 T 实现 Display 和 PartialOrd 时，Pair<T> 才拥有 cmp_display 方法。
impl<T> Pair<T>
where
    T: std::fmt::Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest member is x = {}", self.x);
        } else {
            println!("largest member is y = {}", self.y);
        }
    }
}
