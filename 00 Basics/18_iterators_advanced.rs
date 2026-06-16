fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // iter() 产生不可变引用迭代器：Item = &T。
    let sum: i32 = numbers.iter().sum();
    println!("sum by iter = {sum}");

    // into_iter() 消耗集合，通常产生拥有所有权的元素：Item = T。
    let owned_numbers = vec![10, 20, 30];
    let moved_sum: i32 = owned_numbers.into_iter().sum();
    println!("sum by into_iter = {moved_sum}");
    // println!("{:?}", owned_numbers); // 编译错误：集合已被消耗。

    // iter_mut() 产生可变引用迭代器：Item = &mut T。
    let mut scores = vec![80, 90, 70];
    for score in scores.iter_mut() {
        *score += 5;
    }
    println!("scores after bonus = {:?}", scores);

    // 迭代器是惰性的。map/filter 不消费时不会执行。
    let pipeline = numbers
        .iter()
        .inspect(|n| println!("inspect before filter: {n}"))
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n);

    println!("pipeline has not run until collect");
    let even_squares: Vec<i32> = pipeline.collect();
    println!("even_squares = {:?}", even_squares);

    // 常用适配器：enumerate、zip、take、skip。
    let names = ["Alice", "Bob", "Carol"];
    for (index, name) in names.iter().enumerate() {
        println!("{index}: {name}");
    }

    let ids = [1, 2, 3];
    let pairs: Vec<_> = ids.iter().zip(names.iter()).collect();
    println!("pairs = {:?}", pairs);

    let window: Vec<_> = (1..=10).skip(3).take(4).collect();
    println!("skip 3 then take 4 = {:?}", window);

    // fold 可以表达“从一个初始值开始累计”。
    let product = (1..=5).fold(1, |acc, n| acc * n);
    println!("product 1..=5 = {product}");

    // any/all/find/position 用于查询。
    println!("has even? {}", numbers.iter().any(|n| n % 2 == 0));
    println!("all positive? {}", numbers.iter().all(|n| *n > 0));
    println!("first > 3 = {:?}", numbers.iter().find(|&&n| n > 3));
    println!("position of 4 = {:?}", numbers.iter().position(|&n| n == 4));

    // collect 可以收集到不同集合，目标类型常需要标注。
    let text = "hello rust hello iterator";
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("words = {:?}", words);

    // 从迭代器创建 HashMap。
    use std::collections::HashMap;
    let word_lengths: HashMap<&str, usize> = words.iter().map(|word| (*word, word.len())).collect();
    println!("word lengths = {:?}", word_lengths);
}
