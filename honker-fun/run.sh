#!/bin/bash
set -e
rm -f app.db app.db-shm app.db-wal
cargo run
