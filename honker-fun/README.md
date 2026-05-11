### honker-fun

POC of [honker.dev](https://honker.dev/) — a SQLite-backed durable queue —
using the `honker` Rust crate on edition 2024.

### Build

```bash
cargo build
```

### Run

```bash
./run.sh
```

### Result

```
enqueued job id=1 to=alice@example.com
enqueued job id=2 to=bob@example.com
enqueued job id=3 to=carol@example.com
processing id=1 attempts=1 worker=worker-1 payload={"subject":"hello from honker","to":"alice@example.com"}
  ack=true
processing id=2 attempts=1 worker=worker-1 payload={"subject":"hello from honker","to":"bob@example.com"}
  ack=true
processing id=3 attempts=1 worker=worker-1 payload={"subject":"hello from honker","to":"carol@example.com"}
  ack=true
processed 3 jobs
```
