fn main() {
    let mut v = vec![10,20,30,40,50];
    v.extend([60,70,80].iter());
    println!("{:?}",v);

    let mut strings = vec!["hello".to_string(), "dolly".to_string()];
    strings.extend(["you","are","fine"].iter().map(|s| s.to_string()));
    println!("{:?}",strings);
}
