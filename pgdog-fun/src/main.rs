use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    let conn_str = "host=localhost port=6432 user=pgdog password=pgdog dbname=pgdog";
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls)
        .await
        .expect("Failed to connect to pgdog proxy");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id BIGINT PRIMARY KEY,
                email VARCHAR NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW()
            )",
            &[],
        )
        .await
        .expect("Failed to create users table");

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS payments (
                id BIGINT PRIMARY KEY,
                user_id BIGINT NOT NULL REFERENCES users(id),
                amount DOUBLE PRECISION DEFAULT 0.0,
                created_at TIMESTAMPTZ DEFAULT NOW()
            )",
            &[],
        )
        .await
        .expect("Failed to create payments table");

    client
        .execute("DELETE FROM payments", &[])
        .await
        .expect("Failed to clean payments");
    client
        .execute("DELETE FROM users", &[])
        .await
        .expect("Failed to clean users");

    client
        .execute(
            "INSERT INTO users (id, email) VALUES (1, 'alice@pgdog.dev')",
            &[],
        )
        .await
        .expect("Failed to insert user 1");

    client
        .execute(
            "INSERT INTO users (id, email) VALUES (2, 'bob@pgdog.dev')",
            &[],
        )
        .await
        .expect("Failed to insert user 2");

    client
        .execute(
            "INSERT INTO payments (id, user_id, amount) VALUES (1, 1, 99.99)",
            &[],
        )
        .await
        .expect("Failed to insert payment 1");

    client
        .execute(
            "INSERT INTO payments (id, user_id, amount) VALUES (2, 2, 49.50)",
            &[],
        )
        .await
        .expect("Failed to insert payment 2");

    println!("=== All Users ===");
    let rows = client
        .query("SELECT id, email FROM users ORDER BY id", &[])
        .await
        .expect("Failed to query users");
    for row in &rows {
        let id: i64 = row.get(0);
        let email: &str = row.get(1);
        println!("  User {{ id: {}, email: {} }}", id, email);
    }

    println!("\n=== All Payments ===");
    let rows = client
        .query("SELECT id, user_id, amount FROM payments ORDER BY id", &[])
        .await
        .expect("Failed to query payments");
    for row in &rows {
        let id: i64 = row.get(0);
        let user_id: i64 = row.get(1);
        let amount: f64 = row.get(2);
        println!(
            "  Payment {{ id: {}, user_id: {}, amount: {:.2} }}",
            id, user_id, amount
        );
    }

    println!("\n=== Users with Payments (JOIN) ===");
    let rows = client
        .query(
            "SELECT u.email, p.amount FROM users u JOIN payments p ON u.id = p.user_id ORDER BY u.id",
            &[],
        )
        .await
        .expect("Failed to query join");
    for row in &rows {
        let email: &str = row.get(0);
        let amount: f64 = row.get(1);
        println!("  {} -> ${:.2}", email, amount);
    }

    println!("\nDone - pgdog proxy is working.");
}
