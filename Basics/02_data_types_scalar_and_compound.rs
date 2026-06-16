fn main() {
    // 标量类型：整数、浮点数、布尔值、字符。

    // 整数默认是 i32；可以显式指定 i8/i16/i32/i64/i128/isize 或无符号 u*。
    let signed: i32 = -42;
    let unsigned: u64 = 42;
    let readable = 1_000_000;
    println!("signed = {signed}, unsigned = {unsigned}, readable = {readable}");

    // 浮点数默认是 f64，也可以使用 f32。
    let pi: f64 = 3.1415926;
    let temperature: f32 = 36.5;
    println!("pi = {pi}, temperature = {temperature}");

    // 布尔值用于条件判断。
    let is_learning = true;
    println!("is_learning = {is_learning}");

    // char 表示一个 Unicode 标量值，占 4 字节，不等同于一个字节。
    let letter = 'R';
    let emoji = '🦀';
    println!("letter = {letter}, emoji = {emoji}");

    // 复合类型：元组和数组。

    // 元组可以组合不同类型的值，长度固定。
    let person: (&str, i32, bool) = ("Alice", 30, true);
    let (name, age, active) = person; // 解构元组。
    println!("{name}: age = {age}, active = {active}");
    println!("tuple access by index: {}", person.0);

    // 单元类型 () 表示“没有有意义的值”，常见于不返回值的函数。
    let unit_value = ();
    println!("unit value = {:?}", unit_value);

    // 数组中的元素类型必须相同，长度固定。
    let numbers: [i32; 4] = [10, 20, 30, 40];
    println!("first = {}, len = {}", numbers[0], numbers.len());

    // [初始值; 长度] 可以创建重复元素数组。
    let zeros = [0; 5];
    println!("zeros = {:?}", zeros);

    // 越界访问会 panic。下面这行取消注释会在运行时报错。
    // println!("{}", numbers[99]);
}
