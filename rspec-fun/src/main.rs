#[cfg(test)]
extern crate rspec;

fn main() {
    println!("2+2={}", add(2, 2));
}

fn add(x: u32, y: u32) -> u64 {
    (x + y).into()
}

#[test]
fn test_add() {
    rspec::describe("add", (), |ctx| {
        ctx.when("0 <= x + y <= u32::MAX", |ctx| {
            ctx.then("2 + 4 = 6", |_| assert_eq!(6, add(2, 4)));
            ctx.then("4 + 4 = 8", |_| assert_eq!(8, add(4, 4)));
        });
        ctx.then("is associative", |_| {
            assert_eq!(add(2, 1), add(1, 2));
            assert_eq!(add(4, 1), add(1, 4));
            assert_eq!(add(4, 5), add(5, 4));
            assert_eq!(add(12, 1), add(1, 12));
        });
    });
}
