### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

```
❯ ./run.sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/main`
[2025-02-24T01:57:28Z INFO  main] Starting server at http://127.0.0.1:8080
[2025-02-24T01:57:28Z INFO  actix_server::builder] starting 12 workers
[2025-02-24T01:57:28Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2025-02-24T01:57:28Z INFO  actix_server::server] starting service: "actix-web-service-127.0.0.1:8080", workers: 12, listening on: 127.0.0.1:8080
```

### Result

```
❯ cargo test
   Compiling main v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/actix-web-tokio-rest)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.57s
     Running unittests src/main.rs (target/debug/deps/main-6d3ce150afe05eb7)

running 2 tests
test tests::test_list_cats ... ok
test tests::test_add_cat ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
http://localhost:8080/cats
```

```
[
  {
    "name": "Bobcat",
    "cat_type": "wild",
    "fun_fact": "hunt bugs at night"
  }
]
```

```
./run.sh
```

second terminal
```
❯ ./test.sh
HTTP/1.1 200 OK
content-length: 69
content-type: application/json
date: Mon, 24 Feb 2025 01:50:35 GMT

[{"name":"Bobcat","cat_type":"wild","fun_fact":"hunt bugs at night"}]HTTP/1.1 201 Created
content-length: 132
content-type: application/json
date: Mon, 24 Feb 2025 01:50:35 GMT

[{"name":"Bobcat","cat_type":"wild","fun_fact":"hunt bugs at night"},{"name":"Tiger","cat_type":"wild","fun_fact":"hunts at night"}]%
```