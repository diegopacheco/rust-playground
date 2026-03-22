### Build

```bash
cargo build
```

### Run

```bash
./run.sh
```

### Visualize Traces

Open https://dial9-tokio-telemetry.russell-r-cohen.workers.dev/ and drag in the `.bin` file from `target/traces/`.

### Result

```
Task 0 started
Task 3 started
Task 1 started
Task 2 started
Task 4 started
Task 0 done
Result: 0
Task 1 done
Result: 1
Task 2 done
Result: 4
Task 3 done
Result: 9
Task 4 done
Result: 16
All tasks completed. Traces written to target/traces/
```
