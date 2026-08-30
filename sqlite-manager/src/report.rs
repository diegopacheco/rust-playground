pub fn count(amount: usize, noun: &str) -> String {
    if amount == 1 {
        return format!("{} {noun}", separate(amount));
    }
    let suffix = if noun.ends_with(['s', 'x', 'z']) || noun.ends_with("ch") || noun.ends_with("sh")
    {
        "es"
    } else {
        "s"
    };
    format!("{} {noun}{suffix}", separate(amount))
}

pub fn separate(amount: usize) -> String {
    let digits = amount.to_string();
    let mut text = String::with_capacity(digits.len() + digits.len() / 3);
    for (position, digit) in digits.chars().enumerate() {
        if position > 0 && (digits.len() - position).is_multiple_of(3) {
            text.push(',');
        }
        text.push(digit);
    }
    text
}

pub fn human(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

pub fn size(path: &std::path::Path) -> String {
    std::fs::metadata(path)
        .map(|meta| human(meta.len()))
        .unwrap_or_else(|_| "?".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_item_is_not_pluralised() {
        assert_eq!(count(1, "row"), "1 row");
        assert_eq!(count(1, "index"), "1 index");
    }

    #[test]
    fn words_ending_in_a_sibilant_take_es() {
        assert_eq!(count(2, "index"), "2 indexes");
        assert_eq!(count(0, "table"), "0 tables");
    }

    #[test]
    fn large_counts_stay_readable() {
        assert_eq!(separate(185500), "185,500");
        assert_eq!(separate(1000), "1,000");
        assert_eq!(separate(999), "999");
    }

    #[test]
    fn sizes_step_through_units() {
        assert_eq!(human(512), "512 B");
        assert_eq!(human(2048), "2.0 KB");
    }
}
