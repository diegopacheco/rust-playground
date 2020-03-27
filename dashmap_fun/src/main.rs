use dashmap::DashMap;

fn main() {
    let dm = DashMap::new();
    dm.insert(0, "Hello World");
    dm.insert(1, "Rust");
    dm.insert(2, "Language");

    println!("key 0 == {}",dm.get(&0).unwrap().value());
    println!("Containts key 2 == {} ", dm.contains_key(&2));

    for (k, v) in dm.into_iter().enumerate() {
        println!("key: {} Value: {:?}",k,v);
    }

}
