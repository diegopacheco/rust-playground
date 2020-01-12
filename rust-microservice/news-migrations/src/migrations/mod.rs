extern crate postgres;
extern crate barrel;

use postgres::Client;
use barrel::migration::Migration;
use barrel::types;
use barrel::backend::Pg;

pub trait NewsMigration {
    fn new() -> Self;
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error>;
}

pub struct CreateTableNewsMigration {}
impl NewsMigration for CreateTableNewsMigration {
    fn new() -> Self {
        CreateTableNewsMigration {}
    }
    
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error> {
        let mut m = Migration::new();
        m.create_table("news", |t| {
            t.add_column("id", types::uuid().primary(true));
            t.add_column("desc", types::text());
            t.add_column("url", types::text());
        });
        let news_table = m.make::<Pg>().to_owned();
        println!("Table {} will be created",news_table);
    
        pg_client.execute(&news_table[..],&[])
    }
} 

pub struct AddNewsRecordsMigration {}
impl NewsMigration for AddNewsRecordsMigration {
    fn new() -> Self {
        AddNewsRecordsMigration {}
    }
    
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error> {
        pg_client.execute("INSERT INTO news VALUES 
        (uuid_in(md5(random()::text || clock_timestamp()::text)::cstring),$1,$2)",
                &[&"google",&"google.com"])
    }
} 