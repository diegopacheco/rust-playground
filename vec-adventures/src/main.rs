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

   let r5 = vec!["Diego".to_string(),"Clara".to_string(),"Melina".to_string()];
   println!("has Clara == {}",vec_contains_string("Clara".to_string(),r5.clone()));
   println!("has Dino  == {}",vec_contains_string("Dino".to_string(),r5).clone());

   let mut r6 = vec![1,2,3,4,5,6,7,8,9];
   let r6_arg:&mut Vec<i32> = r6.as_mut();
   let (even, odd) = vec_partition(r6_arg);
   println!("partition on vec<i32> == evens {:?} odds {:?}",even,odd);
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

fn vec_contains_string(search:String,vector:Vec<String>) -> bool {
    for s in vector{
        match s == search {
            true => return true,
            _ => {}
        }
    }
    false
}

fn vec_partition(a: &mut Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let (even, odd): (Vec<i32>, Vec<i32>) = (&*a).into_iter().partition(|&n| n % 2 == 0);
    (even, odd)
}