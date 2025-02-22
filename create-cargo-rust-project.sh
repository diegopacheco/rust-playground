#!/bin/bash

mkdir src/
touch Cargo.toml
touch README.md

touch src/main.rs
echo "fn main() {
    println!(\"Hello, world!\");
}" > src/main.rs

echo '### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Result

```
```

' > README.md

echo "[package]
name = \"main\"
version = \"1.0.0\"
authors = [\"Diego.Pacheco\"]
edition = \"2024\"

[dependencies]
tokio = { version = \"1.36\", features = [\"full\"] } " > Cargo.toml

echo '#!/bin/bash

cargo run' > run.sh
chmod +x ./run.sh