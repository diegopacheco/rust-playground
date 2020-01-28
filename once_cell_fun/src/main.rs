use once_cell::sync::OnceCell;

static mut INSTANCE:OnceCell<i32> = OnceCell::new();

fn main() {
    unsafe{
        INSTANCE.set(42).unwrap();
        printme(INSTANCE.get().unwrap());
        printme(INSTANCE.get().unwrap());
    }
}

fn printme(v:&i32){
    println!("{}",v);
}
