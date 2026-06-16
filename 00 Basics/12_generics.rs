// 泛型让函数和类型可以处理多种具体类型，同时保持类型安全。

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<X, Y>(self, other: MixedPoint<X, Y>) -> MixedPoint<T, Y> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer point = {:?}, x = {}", integer, integer.x());
    println!(
        "float point = {:?}, distance = {}",
        float,
        float.distance_from_origin()
    );

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("mixed point = {:?}", p3);

    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest number = {}", largest(&numbers));
    println!("largest char = {}", largest(&chars));
}

// T: PartialOrd + Copy 是 trait bound：
// T 必须能比较大小，并且能按值复制。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
