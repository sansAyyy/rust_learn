macro_rules! my_vec {
    ($($value:expr),* $(,)?) => {{
        let mut temp = Vec::new();
        $(
            temp.push($value);
        )*
        temp
    }};
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

macro_rules! assert_all {
    ($($condition:expr),+ $(,)?) => {
        $(
            assert!($condition, "assertion failed: {}", stringify!($condition));
        )+
    };
}

fn main() {
    // $()* 表示重复 0 次或多次，$()+ 表示重复 1 次或多次。
    // $(,)? 允许最后多一个逗号。
    let numbers = my_vec![1, 2, 3, 4,];
    println!("numbers = {:?}", numbers);

    let scores = hashmap! {
        "Alice" => 90,
        "Bob" => 80,
    };
    println!("scores = {:?}", scores);

    assert_all!(numbers.len() == 4, scores.contains_key("Alice"),);

    println!("all macro assertions passed");
}
