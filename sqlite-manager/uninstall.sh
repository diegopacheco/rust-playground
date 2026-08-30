#!/bin/bash

set -e

BIN_DIR="${BIN_DIR:-/usr/local/bin}"
NAME="sqlite-manager"

if [ ! -f "$BIN_DIR/$NAME" ]; then
  echo "not installed at $BIN_DIR/$NAME"
  exit 0
fi

if [ -w "$BIN_DIR" ]; then
  rm -f "$BIN_DIR/$NAME"
else
  sudo rm -f "$BIN_DIR/$NAME"
fi

echo "removed $BIN_DIR/$NAME"
