### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

`.collect()` can fanout to multiple collections.
```
[0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
[0, 1, 8, 27, 64, 125, 216, 343, 512, 729]
[0, 1, 16, 81, 256, 625, 1296, 2401, 4096, 6561]
```