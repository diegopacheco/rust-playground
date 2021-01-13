fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
        println!("My Raw Pinter value is {} ",*raw_p);
    }
}
