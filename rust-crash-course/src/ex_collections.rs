use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub fn run(){
    let mut v = vec![3,1,2];
    v.sort();
    println!("{:?}", v);
    let mut h: HashMap<&str,i32> = HashMap::new();
    h.insert("a",1);
    h.insert("b",2);
    println!("{}", h.get("a").unwrap());
    let mut s: HashSet<i32> = HashSet::new();
    s.insert(1); s.insert(2);
    println!("{}", s.contains(&2));
    let mut bt: BTreeMap<i32,&str> = BTreeMap::new();
    bt.insert(2,"b"); bt.insert(1,"a");
    for (k,v) in bt { println!("{}:{}", k, v); }
    let mut bs = BTreeSet::new();
    bs.insert(3); bs.insert(1);
    for x in bs { println!("{}", x); }
}
