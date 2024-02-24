use std::{collections::HashMap, str::FromStr};

#[allow(unused_mut)]
fn main() {
    let mut map = get_map();
    println!("Map: {:?}",map);

    if let Some(v) = map.get("two") {
        let res: String = format!("{}{}", 
                v.to_owned(),
                String::from_str(" portugues").unwrap());
        assert_eq!(res, "dois portugues");
        println!("got [{res}]");
    }

    map = update(map.clone());
    println!("Map: {:?}", map.clone());
}

fn get_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("one","um");
    map.insert("two","dois");
    map.insert("three","tres");
    return map;
}

fn update(mut map: HashMap<&'static str, &'static str>) -> HashMap<&str, &str>{
    match map.get_mut("three") {
        Some(mref) => *mref = "3",
        None => panic!("panic! panic! panic!")
    }
    return map;
}