fn main() {
    // 闭包 closure 是可以存入变量、作为参数传递的匿名函数。
    let add_one = |x: i32| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 闭包通常可以自动推断参数和返回值类型。
    let double = |x| x * 2;
    println!("double(10) = {}", double(10));

    // 闭包可以捕获外部环境。
    let factor = 3;
    let multiply = |x: i32| x * factor; // 不可变借用 factor。
    println!("multiply(4) = {}", multiply(4));
    println!("factor is still usable = {factor}");

    // 可变捕获：闭包内部修改外部变量。
    let mut count = 0;
    let mut increase = || {
        count += 1;
        count
    };
    println!("increase = {}", increase());
    println!("increase = {}", increase());
    // println!("{count}"); // 在 increase 后续还要使用前，不能同时借用 count。

    // move 闭包会取得捕获变量的所有权，常用于线程或异步任务。
    let name = String::from("Rust");
    let say_hello = move || {
        println!("Hello, {name}");
    };
    say_hello();
    // println!("{name}"); // 编译错误：name 已移动进闭包。

    // 闭包作为函数参数时，常用 Fn / FnMut / FnOnce trait bound。
    let numbers = vec![1, 2, 3, 4];
    let evens = filter_numbers(&numbers, |n| n % 2 == 0);
    println!("evens = {:?}", evens);

    let greeting = run_once(|| String::from("computed only once"));
    println!("{greeting}");

    let mut total = 0;
    apply_twice_mut(|| {
        total += 10;
    });
    println!("total after FnMut closure = {total}");
}

// Fn：闭包只不可变借用环境，可以调用多次。
fn filter_numbers<F>(numbers: &[i32], predicate: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    numbers
        .iter()
        .copied()
        .filter(|number| predicate(*number))
        .collect()
}

// FnOnce：闭包可能消耗捕获的值，因此只能保证调用一次。
fn run_once<F>(f: F) -> String
where
    F: FnOnce() -> String,
{
    f()
}

// FnMut：闭包可能可变借用环境，可以调用多次，但调用者需要 mut。
fn apply_twice_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}
