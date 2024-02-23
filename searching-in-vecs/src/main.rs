fn main() {
    let v = vec![10,20,30,40,50];
    assert_eq!(v.iter().position(|&i| i == 30).unwrap(), 2);

    let result = v.iter().position(|&i| i == 30).unwrap();
    println!("Vec {:?} search 2 == {result}",v);
}
