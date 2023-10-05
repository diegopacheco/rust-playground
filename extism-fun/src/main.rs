use extism::{Plugin,Context};

fn main() {
    let wasm = include_bytes!("code.wasm");
    let ctx = Context::new();

    let mut plugin = Plugin::new(&ctx, wasm,[], false).unwrap();
    let data = plugin.call("count_vowels", "this is a test").unwrap();

    assert_eq!(data, b"{\"count\": 4}");
    println!("{:?}",data);
}