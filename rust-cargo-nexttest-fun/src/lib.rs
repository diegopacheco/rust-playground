pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

pub fn multiply(left: i32, right: i32) -> i32 {
    left * right
}

pub fn divide(left: i32, right: i32) -> Option<i32> {
    match right {
        0 => None,
        value => Some(left / value),
    }
}

pub fn is_even(value: i32) -> bool {
    value % 2 == 0
}

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.max(min).min(max)
}

pub fn factorial(value: u32) -> u32 {
    (1..=value).product()
}

pub fn reverse(value: &str) -> String {
    value.chars().rev().collect()
}

pub fn title_case(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.flat_map(char::to_lowercase))
                    .collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn count_words(value: &str) -> usize {
    value.split_whitespace().count()
}
