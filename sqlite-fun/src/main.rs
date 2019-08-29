fn main() {
    println!("SQL Lite + Rust ");

    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users VALUES ('Alice', 42);
            INSERT INTO users VALUES ('Bob', 69);
            ",
        )
    .unwrap();

    connection
    .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
        for &(column, value) in pairs.iter() {
            println!("{} = {}", column, value.unwrap());
        }
        true
    })
    .unwrap();
}
