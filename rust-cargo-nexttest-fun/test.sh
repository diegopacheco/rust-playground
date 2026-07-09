#!/usr/bin/env bash
set -euo pipefail
if ! cargo nextest --version >/dev/null 2>&1; then
  cargo install cargo-nextest --locked
fi
cargo nextest run
