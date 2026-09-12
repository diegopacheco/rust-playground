#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCRIPTS="$ROOT/scripts"
RUN="$ROOT/.run"
LOGS="$RUN/logs"
cd "$ROOT"

mkdir -p "$RUN" "$LOGS"

BIN_NAME="tetris-tui"

log() {
  printf "%s\n" "$*"
}

fail() {
  printf "%s\n" "$*" >&2
  exit 1
}

have() {
  command -v "$1" >/dev/null 2>&1
}

pretty_available() {
  have cargo-pretty && [ -t 1 ]
}

cargo_verb() {
  local verb
  verb="$1"
  shift
  if pretty_available; then
    cargo pretty "$verb" "$@"
  else
    cargo "$verb" "$@"
  fi
}

require_cargo() {
  have cargo || fail "cargo not found; install Rust from https://rustup.rs"
}

release_binary() {
  printf "%s\n" "$ROOT/target/release/$BIN_NAME"
}

debug_binary() {
  printf "%s\n" "$ROOT/target/debug/$BIN_NAME"
}
