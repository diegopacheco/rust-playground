use stoolap::api::Database;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::open_in_memory()?;

    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)", ())?;
    db.execute("INSERT INTO users VALUES (1, 'Alice')", ())?;

    for row in db.query("SELECT * FROM users", ())? {
        let row = row?;
        println!("{}: {}", row.get::<i64>(0)?, row.get::<String>(1)?);
    }

    Ok(())
}