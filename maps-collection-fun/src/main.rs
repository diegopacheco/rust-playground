fn main() {
    let cities = [("California","Sacramento"),("NY","NYC"),("Oregon","Portland")];
    if let Some(val) = cities.iter().find(|t| t.0 == "NY") {
        assert_eq!(val.1,"NYC");
    }
    println!("{:?}",cities);

    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("one","eins");
    map.insert("two","zwei");
    map.insert("three","drei");
    assert_eq! (map.contains_key("two"), true);
    assert_eq! (map.get("two"), Some(&"zwei"));
    println!("{:?}",map);
}
