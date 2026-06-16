use testing_debugging::calculator;

#[test]
#[should_panic(expected = "index out of bounds")]
fn should_panic_example() {
    let values = [1, 2, 3];
    let _ = read_at(&values, 99);
}

#[test]
fn result_returning_test() -> Result<(), String> {
    let value = calculator::divide(20, 4)?;
    assert_eq!(value, 5);
    Ok(())
}

fn read_at(values: &[i32], index: usize) -> i32 {
    values[index]
}
