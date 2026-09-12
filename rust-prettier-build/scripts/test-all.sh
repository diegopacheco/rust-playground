#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

log "checking formatting"
cargo fmt --all --check || fail "formatting check failed; run scripts/lint.sh --fix"

log "running clippy"
cargo clippy --all-targets -- -D warnings || fail "clippy failed"

log "running tests"
cargo_verb test || fail "tests failed"

log "all suites passed"
