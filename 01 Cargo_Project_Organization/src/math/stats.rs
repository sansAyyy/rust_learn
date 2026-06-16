pub fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let total: f64 = values.iter().sum();
    Some(total / values.len() as f64)
}

pub fn max(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}
