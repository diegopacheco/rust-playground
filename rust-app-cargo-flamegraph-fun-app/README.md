### Install
```bash
sudo apt install linux-tools-common linux-tools-generic linux-tools-`uname -r
cargo install flamegraph
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
```
### Build
```bash
cargo build
cargo run -- release
```
### FlameGraph in action
```bash
cargo flamegraph main
```
