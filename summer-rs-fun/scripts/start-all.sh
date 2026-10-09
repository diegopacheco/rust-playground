#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

log "starting"

require cargo
cargo build || fail "cargo build failed"

start_bg backend "$ROOT" "$ROOT/target/debug/main"
wait_port_up "$(service_port backend)" 60 || fail "backend did not open port $(service_port backend), see $LOGS/backend.log"
log "backend up on $(service_url backend)"

"$SCRIPTS/status.sh"

log "links"
for name in $(service_names); do
  printf "%-14s %s\n" "$name" "$(service_url "$name")"
done
