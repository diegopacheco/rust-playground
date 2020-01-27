extern crate rocksdb;

use rocksdb::DB;

fn main() {
    println!("Hello, world! RocksDB + Rust");

    // NB: db is automatically closed at end of lifetime
    let db = DB::open_default("/tmp/rocksdb/").unwrap();
    db.put(b"my key", b"my value").unwrap();
    match db.get(b"my key") {
      Ok(Some(value)) => println!("retrieved value {}", value.get(0).unwrap()),
      Ok(None) => println!("value not found"),
      Err(e) => println!("operational problem encountered: {}", e),
   }
   db.delete(b"my key").unwrap();
}
