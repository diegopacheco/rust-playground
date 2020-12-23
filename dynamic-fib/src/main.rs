use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    for i in 1..41{
        println!("{}: {}",i,dynamic_fib(i,&mut map));
    }
}

fn dynamic_fib(n:u64,map:&mut HashMap<u64,u64>) -> u64{
    match n {
        0 | 1 => 1,
        n=> {
            if map.contains_key(&n){
                *map.get(&n).unwrap()
            }else{
                let val = dynamic_fib(n-1,map) + dynamic_fib(n-2,map);
                map.insert(n,val);
                val
            }
        }
    }
}