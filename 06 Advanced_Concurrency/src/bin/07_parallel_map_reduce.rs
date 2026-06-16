use std::thread;

fn main() {
    let numbers: Vec<u64> = (1..=1_000).collect();
    let total = parallel_sum_of_squares(&numbers, 4);
    println!("sum of squares = {total}");

    let expected: u64 = numbers.iter().map(|n| n * n).sum();
    println!("matches sequential? {}", total == expected);
}

fn parallel_sum_of_squares(numbers: &[u64], chunk_count: usize) -> u64 {
    let chunk_size = numbers.len().div_ceil(chunk_count);

    // thread::scope 允许子线程借用 numbers，不必 clone 或要求 'static。
    thread::scope(|scope| {
        let mut handles = Vec::new();

        for chunk in numbers.chunks(chunk_size) {
            handles.push(scope.spawn(move || chunk.iter().map(|n| n * n).sum::<u64>()));
        }

        handles
            .into_iter()
            .map(|handle| handle.join().expect("worker should finish"))
            .sum()
    })
}
