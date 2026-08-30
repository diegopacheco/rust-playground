#!/bin/bash

set -e
cargo build --release
./target/release/sqlite-manager "$@"
