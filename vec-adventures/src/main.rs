use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::Cell;

fn main() {

    let r1:VecDeque<i32> = to_vecdeque(vec![1,2,3]);
    println!("vec to VecDeque {:?}",r1);

    let r2:Rc<Cell<Vec<&str>>> = to_str_vec(vec!["a".to_string(), "b".to_string()]);
    println!("vec<&str> to Vec<String> {:?}", r2.take());

   // let r3:Vec<&String> = to_string_vec(&vec![&"a", &"b"]);
   // println!("vec<String> to Vec<&str> {:?}", r3);
}

fn to_vecdeque(vector:Vec<i32>) -> VecDeque<i32> {
    VecDeque::from(vector)
}

/*
fn to_str_vec<'a>(v:Vec<String>) -> Vec<&'static str> {
    let mut items = Vec::<&str>::new();
    for item in v {
        items.push(&item.as_str().clone());
    }
    items
}
*/

fn to_str_vec<'a>(v:Vec<String>) -> Rc<Cell<Vec<&'a str>>> {
    let mut items = Vec::<&str>::new();
    for item in v {
        items.push(&item.as_str().clone() );
    }
    Rc::new(Cell::from(items))
}

/*
fn to_string_vec<'a>(v:&Vec<&'a str>) -> Vec<&'a String> {
    let mut items = Vec::<&String>::new();
    for item in v {
        items.push(&item.clone().to_string());
    }
    items
}
*/