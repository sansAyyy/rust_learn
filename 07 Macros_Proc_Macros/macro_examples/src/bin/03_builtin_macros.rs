fn main() {
    // println! / format! 都是宏，因为它们接收可变数量参数并做格式化检查。
    println!("{} + {} = {}", 2, 3, 2 + 3);
    let message = format!("hello {}", "macro");
    println!("{message}");

    // vec! 快速构造 Vec。
    let values = vec![1, 2, 3];
    println!("values = {:?}", values);

    // dbg! 打印表达式、文件和行号，并返回表达式值。
    let doubled = dbg!(values.iter().map(|value| value * 2).collect::<Vec<_>>());
    println!("doubled = {:?}", doubled);

    // matches! 判断表达式是否匹配某个模式。
    let status = Some(200);
    println!("is success? {}", matches!(status, Some(200..=299)));

    // include_str! 在编译期把文件内容嵌入二进制。
    let cargo_toml = include_str!("../../Cargo.toml");
    println!("Cargo.toml bytes = {}", cargo_toml.len());

    // env! 在编译期读取环境变量；option_env! 不存在时返回 None。
    println!("package name = {}", env!("CARGO_PKG_NAME"));
    println!("optional env = {:?}", option_env!("SOME_MISSING_ENV"));
}
