use std::collections::HashSet;
use std::hash::Hash;

trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

impl <T,I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item=T> {
    fn to_set(self) -> HashSet<T> {
       self.collect()
    }
}

fn main() {
    let fruit = get_by_space_from_str("apple orange pear");
    let colours = get_by_space_from_str("brown purple orange yellow");
    println!("{:?}", colours);
    println!("{:?}", fruit);
    find_intersection(fruit.clone(), colours.clone());

    let intersect = fruit.intersection(&colours).to_set();
    println!("{:?}", intersect);
}

fn get_by_space_from_str(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn find_intersection(fruit:HashSet<&str>,colours:HashSet<&str>) {
    for c in fruit.intersection(&colours) {
        println!("{:?}",c);
    }
}
