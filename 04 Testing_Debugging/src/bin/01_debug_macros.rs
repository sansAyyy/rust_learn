use testing_debugging::calculator;

fn main() {
    let values = vec![2, 4, 6, 8];

    // dbg! 会打印表达式、文件和行号到 stderr，并返回表达式的值。
    let average = dbg!(calculator::average(&values));
    println!("average = {:?}", average);

    // eprintln! 适合临时调试输出，不会混入 stdout 的正式输出。
    eprintln!("debug values = {:?}", values);

    let total: i32 = values.iter().sum();
    println!("total = {total}");

    // 调试完成后，通常删除 dbg! 和临时 eprintln!。
}
