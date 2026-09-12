#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

MODE="${1:-debug}"

case "$MODE" in
  debug)
    cargo_verb build
    log "built $(debug_binary)"
    ;;
  release)
    cargo_verb build --release
    log "built $(release_binary)"
    ;;
  *)
    fail "usage: build.sh [debug|release]"
    ;;
esac
