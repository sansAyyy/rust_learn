#[derive(Debug)]
struct Account {
    owner: String,
    balance: i64,
}

fn main() {
    // 借用拆分：编译器能理解结构体不同字段可以分别可变借用。
    let mut account = Account {
        owner: String::from("Alice"),
        balance: 100,
    };

    let owner = &mut account.owner;
    let balance = &mut account.balance;
    owner.push_str(" Smith");
    *balance += 50;
    println!("account after split borrow = {:?}", account);

    // 对数组/切片，不能直接同时借用两个索引，因为编译器不总能证明它们不同。
    let mut numbers = [1, 2, 3, 4, 5];
    let (left, right) = numbers.split_at_mut(2);
    left[0] += 10;
    right[0] += 20;
    println!("numbers after split_at_mut = {:?}", numbers);

    // 非词法生命周期 NLL：引用最后一次使用后，借用可以提前结束。
    let mut text = String::from("hello");
    let first = &text;
    println!("first immutable borrow = {first}");
    text.push_str(" world"); // first 最后一次使用已经结束，所以这里可变借用合法。
    println!("text after mutation = {text}");

    // 重借用 reborrow：从 &mut T 临时创建更短生命周期的 &T 或 &mut T。
    let mut value = 10;
    let mutable_ref = &mut value;
    print_readonly(&*mutable_ref); // 把 &mut i32 重借用为 &i32。
    add_one(mutable_ref); // 原来的可变引用之后还能继续使用。
    println!("value after reborrow = {value}");

    // 通过函数缩短借用范围，常用于解决“借太久”的问题。
    let mut names = vec![String::from("alice"), String::from("bob")];
    uppercase_first(&mut names);
    names.push(String::from("carol"));
    println!("names = {:?}", names);
}

fn print_readonly(value: &i32) {
    println!("readonly value = {value}");
}

fn add_one(value: &mut i32) {
    *value += 1;
}

fn uppercase_first(values: &mut [String]) {
    if let Some(first) = values.first_mut() {
        first.make_ascii_uppercase();
    }
}
