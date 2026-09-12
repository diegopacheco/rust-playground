#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

log "removing build artifacts"
cargo clean
rm -rf "$RUN"
log "clean"
