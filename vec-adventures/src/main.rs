use std::collections::VecDeque;

fn main() {

    let r1:VecDeque<i32> = to_vecdeque(vec![1,2,3]);
    println!("vec to VecDeque {:?}",r1);

    let a = "a".to_string();
    let b = "b".to_string();
    let r2_args = &vec![&a, &b];
    let r2:Vec<&str> = to_str_vec(r2_args);
    println!("vec<String> to Vec<&str> {:?}", r2);

   let r3:Vec<String> = to_string_vec(vec![&"c", &"d"]);
   println!("vec<&str> to Vec<String> {:?}", r3);

   let r4 = vec!["a".to_string(),"e".to_string(),"x".to_string()];
   println!("vec<String> fp == {:?}", func_prog(r4));
}

fn to_vecdeque(vector:Vec<i32>) -> VecDeque<i32> {
    VecDeque::from(vector)
}

fn to_str_vec<'a>(v:&Vec<&'a String>) -> Vec<&'a str> {
    let mut items = Vec::<&str>::new();
    for item in v {
        items.push(item);
    }
    items
}

fn to_string_vec<'a>(v:Vec<&'a str>) -> Vec<String> {
    let mut items = Vec::<String>::new();
    for item in v {
        items.push(item.to_string());
    }
    items
}

fn func_prog(letters:Vec<String>) -> String {
    let result: Vec<String> = letters.iter()
        .filter(|l|    **l=="a".to_string()
                            || **l=="e".to_string()
                            || **l=="i".to_string()
                            || **l=="o" .to_string()
                            || **l=="u".to_string()
        ).map(|v| v.to_uppercase())
        .collect::<Vec<String>>();
    result.join(",")
}