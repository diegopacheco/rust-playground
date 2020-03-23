fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()
        .fold(0.0, |a, b| a + b);
    let result = total / values.len() as f64;
    return result;
}

pub fn main_mod() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}