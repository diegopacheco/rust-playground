### Dependencies

```bash
sudo apt-get install libmysqlclient-dev
```

### Install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features mysql
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
Displaying 2 users
Person { id: 1, first_name: "John", last_name: "Doe", created_at: 2025-02-24T06:31:50 }
Person { id: 2, first_name: "Jane", last_name: "Doe", created_at: 2025-02-24T06:31:50 }
Done.
```

```
mysql> select * from person;
+----+------------+-----------+---------------------+
| id | first_name | last_name | created_at          |
+----+------------+-----------+---------------------+
|  1 | John       | Doe       | 2025-02-24 06:31:50 |
|  2 | Jane       | Doe       | 2025-02-24 06:31:50 |
+----+------------+-----------+---------------------+
2 rows in set (0.00 sec)

mysql> 
```