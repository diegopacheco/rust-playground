pub fn execute(){
    let data = [1,2,3,4,5,6];
    use_slice(&data[1..4]);
}

fn use_slice(slice: &[i32]){
    println!("{:?}", slice);
}