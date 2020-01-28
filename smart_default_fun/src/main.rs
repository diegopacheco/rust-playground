#[macro_use] extern crate smart_default;

#[derive(SmartDefault)]
enum Foo {
    Bar,
    #[default]
    Baz {
        #[default = 12]
        a: i32,
        b: i32,
        #[default(Some(Default::default()))]
        c: Option<i32>,
        #[default(_code = "vec![1, 2, 3]")]
        d: Vec<u32>,
        #[default = "four"]
        e: String
    }
}

fn main() {
    let _b = Foo::Baz {
        a: 12,
        b: 0,
        c: Some(0),
        d: vec![1, 2, 3],
        e: "four".to_owned(),
    };
}
