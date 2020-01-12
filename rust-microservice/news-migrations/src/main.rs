extern crate barrel;

use barrel::migration::Migration;
use barrel::types;
use barrel::backend::Pg;

fn main() {

    let mut m = Migration::new();
    m.create_table("news", |t| {
        t.add_column("id", types::primary());
        t.add_column("desc", types::varchar(255));
        t.add_column("url", types::varchar(255));
    });
    println!("{}", m.make::<Pg>());
}