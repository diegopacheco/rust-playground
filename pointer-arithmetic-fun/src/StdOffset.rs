pub fn compute(){
    let items = [1usize, 2, 3, 4];
    let ptr = &items[1] as *const usize;

    println!("should be 2 == {}", unsafe { *ptr });
    println!("should be 1 == {}", unsafe { *ptr.offset(-1) });
    println!("should be 3 == {}", unsafe { *ptr.offset(1) });
    println!("should be 4 == {}", unsafe { *ptr.offset(2) });
}