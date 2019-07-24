pub fn execute(){
    let mut data = [1,2,3,4,5,6];
    useSlice(&data[1..4]);
}

fn useSlice(slice: &[i32]){
    println!("{:?}", slice);
}