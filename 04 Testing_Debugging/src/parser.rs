use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub struct UserId(pub u64);

pub fn parse_user_id(input: &str) -> Result<UserId, ParseIntError> {
    input.trim().parse::<u64>().map(UserId)
}

pub fn parse_csv_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input
        .split(',')
        .map(|part| part.trim().parse::<i32>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_user_id_with_spaces() {
        assert_eq!(parse_user_id(" 42 "), Ok(UserId(42)));
    }

    #[test]
    fn parses_csv_numbers() {
        assert_eq!(parse_csv_numbers("1, 2, 3"), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn csv_parse_error_is_returned() {
        assert!(parse_csv_numbers("1, x, 3").is_err());
    }
}
