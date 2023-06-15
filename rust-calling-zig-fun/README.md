### Build Zig Lib
```bash
zig build-lib add.zig -dynamic
```

### Build Rust Program using Zig Lib
```bash
rustc main.rs -L . -l add
```

### Run
```bash
export LD_LIBRARY_PATH=./
./main
```
will output
```bash
./main
1 + 2 = 3
```