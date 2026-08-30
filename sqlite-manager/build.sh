#!/bin/bash

set -e
cd "$(dirname "$0")"

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release

echo "built target/release/sqlite-manager"
