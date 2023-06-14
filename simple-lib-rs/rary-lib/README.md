### Build
```bash
rustc --crate-type=lib rary.rs
```
### Build the App link the lib
```bash
rustc main.rs --extern rary=library.rlib
./main
```