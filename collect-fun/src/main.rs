use std::collections::HashSet;

fn main() {
    let a_to_z_set = ('a'..='z').collect::<HashSet<_>>();

    println!("{:?}", a_to_z_set);
    println!("{:?}", a_to_z_set.get(&'a'));

    for letter in &a_to_z_set {
        print!("{}", letter);
    }
}
