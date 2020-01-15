extern crate chrono;

use chrono::prelude::*;
use chrono::offset::LocalResult;

fn main() {
    let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
    assert_eq!(dt, Utc.yo(2014, 189).and_hms(9, 10, 11)); // July 8 is 188th day of the year 2014 (`o` for "ordinal")

    // dynamic verification
    assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33), LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33)));

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");

    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));
    assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),Ok(fixed_dt.clone()));

    let local: DateTime<Local> = Local::now();
    println!("Now: {}",local);
}
