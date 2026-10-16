#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

PORT="$(service_port app)"
if port_up "$PORT"; then
  log "app already running"
else
  cargo build
  PORT="$PORT" start_bg app "$ROOT" "$ROOT/target/debug/proa-fun"
  wait_port_up "$PORT" 60 || fail "app did not open port $PORT, see $LOGS/app.log"
fi
log "app        $(service_url app)"
log "markdown   $(service_url app)/index.md"
log "health     $(service_url app)/healthz"
