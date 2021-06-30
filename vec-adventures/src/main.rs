use std::collections::VecDeque;
use std::borrow::Borrow;

fn main() {
    println!("vec to VecDeque {:?}",to_vecdeque(vec![1,2,3]));
    println!("vec<&str> to Vec<String> {:?}",to_vec(vec!["a".to_string(),"b".to_string()]));
}

fn to_vecdeque(vector:Vec<i32>) -> VecDeque<i32> {
    VecDeque::from(vector)
}

fn to_vec(v:Vec<String>) -> Vec<&'static str>{
    let mut items = Vec::<&str>::with_capacity(v.len());
    for item in v {
        items.push(item.clone().as_str());
    }
    items
}
