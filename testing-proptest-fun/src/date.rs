use proptest::prelude::*;

proptest! {
    #[test]
    fn parses_date_back_to_original(y in 0u32..10000,
                                    m in 1u32..13, d in 1u32..32) {
        println!("y = {}, m = {}, d = {}", y, m, d);
        let (y2, m2, d2) = parse_date(
            &format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();
        // prop_assert_eq! is basically the same as assert_eq!, but doesn't
        // cause a bunch of panic messages to be printed on intermediate
        // test failures. Which one to use is largely a matter of taste.
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }

    #[test]
    fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
        parse_date(&s).unwrap();
    }
}

#[test]
fn test_october_first() {
    assert_eq!(Some((0, 10, 1)), parse_date("0000-10-01"));
}

pub fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() { return None; }

    // NEW: Ignore non-ASCII strings so we don't need to deal with Unicode.
    if !s.is_ascii() { return None; }

    if "-" != &s[4..5] || "-" != &s[7..8] { return None; }

    let year = &s[0..4];
    //let month = &s[6..7];
    let month = &s[5..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(
        |y| month.parse::<u32>().ok().and_then(
            |m| day.parse::<u32>().ok().map(
                |d| (y, m, d))))
}