extern crate case;

use case::CaseExt;

fn main() {

// Snake case operations:
assert_eq!(&"a_string_and_a_miss".to_camel(), "AStringAndAMiss");
assert_eq!(&"string_henry_iii".to_camel_lowercase(), "stringHenryIii");
assert_eq!(&"stringing_in_the_rain".to_dashed(), "stringing-in-the-rain");

// Camel case operations:
assert_eq!(&"martinLutherStringJr".to_snake(), "martin_luther_string_jr");
assert!(&"martinLutherStringJr".is_camel_lowercase());

// Universal operations:
assert_eq!(&"stringy string".to_capitalized(), "Stringy string");
assert!(&"Stringy string".is_capitalized());

}
