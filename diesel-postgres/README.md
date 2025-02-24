### Dependencies

```bash
sudo apt-get install postgresql postgresql-contrib libpq-dev
```

### Install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Setup DB and run migrations

```bash
diesel setup
diesel migration generate create_users
diesel migration run
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

```
❯ diesel setup

Running migration 2025-23-02-21-00-000_create_users
```

```
❯ diesel migration generate create_users

Creating migrations/2025-02-24-054628_create_users/up.sql
Creating migrations/2025-02-24-054628_create_users/down.sql
```

```
❯ diesel migration run
Running migration 2025-02-24-054628_create_users
```

```
❯ ./run.sh
   Compiling main v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/diesel)
warning: fields `id`, `name`, and `hair_color` are never read
  --> src/main.rs:20:5
   |
19 | struct User {
   |        ---- fields in this struct
20 |     id: i32,
   |     ^^
21 |     name: String,
   |     ^^^^
22 |     hair_color: Option<String>,
   |     ^^^^^^^^^^
   |
   = note: `User` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: `main` (bin "main") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/main`
Displaying 6 users
User { id: 1, name: "Sean", hair_color: Some("Black") }
User { id: 2, name: "Gordon", hair_color: None }
User { id: 3, name: "Sean", hair_color: Some("Black") }
User { id: 4, name: "Gordon", hair_color: None }
User { id: 5, name: "Sean", hair_color: Some("Black") }
User { id: 6, name: "Gordon", hair_color: None }
Done.
```