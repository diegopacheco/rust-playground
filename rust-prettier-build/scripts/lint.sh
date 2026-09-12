#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

if [ "${1:-}" = "--fix" ]; then
  log "formatting sources"
  cargo fmt --all
  log "applying clippy suggestions"
  cargo clippy --fix --allow-dirty --allow-staged --all-targets
  log "lint fixes applied"
else
  cargo fmt --all --check || fail "formatting check failed; run lint.sh --fix"
  cargo clippy --all-targets -- -D warnings || fail "clippy failed"
  log "lint clean"
fi
