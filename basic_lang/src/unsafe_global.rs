
static mut COUNTER:i8 = 1;

pub fn execute(){
    
    unsafe{
        println!("\nKids this is unsafe == {}", COUNTER);
    }
}