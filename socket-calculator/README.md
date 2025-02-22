### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

Server

```
❯ ./run-server.sh
   Compiling tcp-calculator v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/socket-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/server`
Server listening on port 8888
New connection established!
```

Client

```
❯ ./run-client.sh
   Compiling tcp-calculator v1.0.0 (/mnt/e35d88d4-42b9-49ea-bf29-c4c3b018d429/diego/git/diegopacheco/rust-playground/socket-calculator)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/client`
Connected to server!
Enter two numbers separated by space (or 'quit' to exit):
20 30
Server response: Result: 20 + 30 = 50
Enter two numbers separated by space (or 'quit' to exit):
5 5
Server response: Result: 5 + 5 = 10
Enter two numbers separated by space (or 'quit' to exit):

```
