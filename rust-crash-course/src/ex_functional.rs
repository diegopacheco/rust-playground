pub fn run(){
    let v = vec![1,2,3,4,5];
    let s: i32 = v.iter().map(|x| x*2).filter(|x| x%3==0).sum();
    println!("{}", s);
    let a = ["a","b","c"]; 
    let j = a.iter().cloned().collect::<Vec<_>>().join("-");
    println!("{}", j);
    let f = |x:i32| x+1; 
    println!("{}", f(9));
}
