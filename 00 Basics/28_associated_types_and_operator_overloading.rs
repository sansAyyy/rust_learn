use std::ops::Add;

// 关联类型把“输出类型”作为 trait 实现的一部分。
trait Parser {
    type Output;

    fn parse(&self, input: &str) -> Option<Self::Output>;
}

struct NumberParser;
struct WordsParser;

impl Parser for NumberParser {
    type Output = i32;

    fn parse(&self, input: &str) -> Option<Self::Output> {
        input.trim().parse().ok()
    }
}

impl Parser for WordsParser {
    type Output = Vec<String>;

    fn parse(&self, input: &str) -> Option<Self::Output> {
        Some(input.split_whitespace().map(str::to_string).collect())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

// Add trait 使用关联类型 Output 来表示加法结果类型。
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

fn main() {
    let number_parser = NumberParser;
    let words_parser = WordsParser;

    let number = parse_and_print(&number_parser, "42");
    let words = parse_and_print(&words_parser, "hello trait system");
    println!("number = {:?}", number);
    println!("words = {:?}", words);

    let length = Millimeters(500) + Meters(2);
    println!("length = {:?}", length);

    // 常见关联类型例子：
    // Iterator::Item 表示每次 next() 产出的元素类型。
    let values = vec![1, 2, 3];
    let mut iter = values.into_iter();
    println!("first item = {:?}", iter.next());
}

fn parse_and_print<P>(parser: &P, input: &str) -> Option<P::Output>
where
    P: Parser,
    P::Output: std::fmt::Debug,
{
    let output = parser.parse(input);
    println!("parsed output = {:?}", output);
    output
}
