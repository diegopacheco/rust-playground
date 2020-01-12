extern crate barrel;
extern crate postgres;

use postgres::Client;
use postgres::tls::NoTls;
use barrel::migration::Migration;
use barrel::types;
use barrel::backend::Pg;

fn main() -> Result<(), postgres::Error> {

    let mut m = Migration::new();
    m.create_table("news", |t| {
        t.add_column("id", types::uuid().primary(true));
        t.add_column("desc", types::text());
        t.add_column("url", types::text());
    });
    let news_table = m.make::<Pg>().to_owned();
    println!("Table {} will be created",news_table);
    
    let mut conn = Client::connect("postgres://postgres:docker@172.17.0.2:5432/postgres", NoTls).unwrap();
    let result = conn.execute(&news_table[..],&[]);
    match result{
        Ok(changes) => print!("Migrations done. Changes Done {}",changes),
        Err(error)  => print!("Error: {}",error),
    };
    
    Ok(())
}