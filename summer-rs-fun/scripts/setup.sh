#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

log "setup started"

require cargo
cargo build || fail "cargo build failed"

if ! grep -q '^\.run/$' "$ROOT/.gitignore" 2>/dev/null; then
  printf ".run/\n" >>"$ROOT/.gitignore"
fi

log "setup done"
