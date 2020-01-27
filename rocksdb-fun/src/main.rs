fn main() {
    println!("Hello, world!");
    // NB: db is automatically closed at end of lifetime
    let db = DB::open_default("/tmp/rocksdb/").unwrap();
    db.put(b"my key", b"my value");
    match db.get(b"my key") {
      Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
      Ok(None) => println!("value not found"),
      Err(e) => println!("operational problem encountered: {}", e),
   }
   db.delete(b"my key").unwrap();
}
