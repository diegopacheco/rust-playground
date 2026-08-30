#!/bin/bash

set -e
cd "$(dirname "$0")"

BIN_DIR="${BIN_DIR:-/usr/local/bin}"
NAME="sqlite-manager"

cargo build --release

if [ ! -d "$BIN_DIR" ]; then
  mkdir -p "$BIN_DIR" 2>/dev/null || sudo mkdir -p "$BIN_DIR"
fi

if [ -w "$BIN_DIR" ]; then
  install -m 755 "target/release/$NAME" "$BIN_DIR/$NAME"
else
  sudo install -m 755 "target/release/$NAME" "$BIN_DIR/$NAME"
fi

echo "installed $BIN_DIR/$NAME"
"$BIN_DIR/$NAME" version
