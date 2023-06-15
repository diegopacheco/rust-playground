extern "C" {
    fn zig_add(a: i32, b: i32) -> i32;
}

fn main() {
    let result = unsafe { 
        zig_add(1, 2)
    };
    println!("1 + 2 = {}", result);
}
