use rust_cargo_nexttest_fun::{
    add, clamp, count_words, divide, factorial, is_even, multiply, reverse, subtract, title_case,
};

#[test]
fn adds_positive_numbers() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn adds_negative_numbers() {
    assert_eq!(add(-2, -3), -5);
}

#[test]
fn adds_mixed_numbers() {
    assert_eq!(add(-2, 3), 1);
}

#[test]
fn subtracts_positive_numbers() {
    assert_eq!(subtract(9, 4), 5);
}

#[test]
fn subtracts_to_negative() {
    assert_eq!(subtract(4, 9), -5);
}

#[test]
fn subtracts_negative_numbers() {
    assert_eq!(subtract(-4, -9), 5);
}

#[test]
fn multiplies_positive_numbers() {
    assert_eq!(multiply(6, 7), 42);
}

#[test]
fn multiplies_by_zero() {
    assert_eq!(multiply(6, 0), 0);
}

#[test]
fn multiplies_negative_numbers() {
    assert_eq!(multiply(-6, -7), 42);
}

#[test]
fn divides_evenly() {
    assert_eq!(divide(20, 5), Some(4));
}

#[test]
fn divides_with_integer_result() {
    assert_eq!(divide(22, 5), Some(4));
}

#[test]
fn skips_division_by_zero() {
    assert_eq!(divide(22, 0), None);
}

#[test]
fn detects_even_number() {
    assert!(is_even(10));
}

#[test]
fn detects_odd_number() {
    assert!(!is_even(11));
}

#[test]
fn detects_negative_even_number() {
    assert!(is_even(-8));
}

#[test]
fn clamps_low_value() {
    assert_eq!(clamp(2, 5, 10), 5);
}

#[test]
fn clamps_high_value() {
    assert_eq!(clamp(20, 5, 10), 10);
}

#[test]
fn keeps_value_inside_bounds() {
    assert_eq!(clamp(7, 5, 10), 7);
}

#[test]
fn factorial_of_zero() {
    assert_eq!(factorial(0), 1);
}

#[test]
fn factorial_of_one() {
    assert_eq!(factorial(1), 1);
}

#[test]
fn factorial_of_five() {
    assert_eq!(factorial(5), 120);
}

#[test]
fn reverses_ascii_text() {
    assert_eq!(reverse("rust"), "tsur");
}

#[test]
fn reverses_empty_text() {
    assert_eq!(reverse(""), "");
}

#[test]
fn reverses_words_with_space() {
    assert_eq!(reverse("ab cd"), "dc ba");
}

#[test]
fn title_cases_single_word() {
    assert_eq!(title_case("rust"), "Rust");
}

#[test]
fn title_cases_multiple_words() {
    assert_eq!(title_case("cargo nextest"), "Cargo Nextest");
}

#[test]
fn title_cases_mixed_case() {
    assert_eq!(title_case("rUsT CaRgO"), "Rust Cargo");
}

#[test]
fn counts_empty_words() {
    assert_eq!(count_words(""), 0);
}

#[test]
fn counts_single_word() {
    assert_eq!(count_words("rust"), 1);
}

#[test]
fn counts_many_words() {
    assert_eq!(count_words("rust cargo nextest"), 3);
}
