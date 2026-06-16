pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn divide(left: i32, right: i32) -> Result<i32, String> {
    if right == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(left / right)
    }
}

pub fn average(values: &[i32]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let sum: i32 = values.iter().sum();
    Some(sum as f64 / values.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn divides_numbers() -> Result<(), String> {
        let value = divide(10, 2)?;
        assert_eq!(value, 5);
        Ok(())
    }

    #[test]
    fn divide_by_zero_returns_error() {
        let error = divide(10, 0).expect_err("division by zero should fail");
        assert_eq!(error, "cannot divide by zero");
    }

    #[test]
    fn average_of_empty_slice_is_none() {
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn average_of_values() {
        assert_eq!(average(&[2, 4, 6]), Some(4.0));
    }
}
