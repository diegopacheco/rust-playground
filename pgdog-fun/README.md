### pgdog-fun

POC with pgdog - a Rust-based PostgreSQL proxy.

Rust app connects to PostgreSQL 17 through pgdog proxy using transaction pooling.

### pgdog features used

* Transaction Pooling - shares postgres connections across clients at transaction boundaries
* Connection Proxying - rust app connects to pgdog(6432) instead of postgres(5432) directly
* User Authentication - pgdog authenticates clients via users.toml before proxying to postgres

### How to run

```bash
./start.sh           # postgres 17 + pgdog proxy up
./run.sh             # rust app connects via pgdog on port 6432
./run-sql-client.sh  # interactive psql through pgdog
./stop.sh            # tear down
```

### Result
```
❯ ./run.sh
   Compiling pgdog-fun v1.0.0 (/Users/diegopacheco/git/diegopacheco/rust-playground/pgdog-fun)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.41s
     Running `target/debug/pgdog-fun`
=== All Users ===
  User { id: 1, email: alice@pgdog.dev }
  User { id: 2, email: bob@pgdog.dev }

=== All Payments ===
  Payment { id: 1, user_id: 1, amount: 99.99 }
  Payment { id: 2, user_id: 2, amount: 49.50 }

=== Users with Payments (JOIN) ===
  alice@pgdog.dev -> $99.99
  bob@pgdog.dev -> $49.50

Done - pgdog proxy is working.
```

```
./run-sql-client.sh
```

```
pgdog=# \dt;
         List of relations
 Schema |   Name   | Type  | Owner
--------+----------+-------+-------
 public | payments | table | pgdog
 public | users    | table | pgdog
(2 rows)

pgdog=# select * from users;
 id |      email      |          created_at
----+-----------------+-------------------------------
  1 | alice@pgdog.dev | 2026-02-23 23:57:39.105288+00
  2 | bob@pgdog.dev   | 2026-02-23 23:57:39.105921+00
(2 rows)

pgdog=# select * from payments;
 id | user_id | amount |          created_at
----+---------+--------+-------------------------------
  1 |       1 |  99.99 | 2026-02-23 23:57:39.106505+00
  2 |       2 |   49.5 | 2026-02-23 23:57:39.107161+00
(2 rows)

```
