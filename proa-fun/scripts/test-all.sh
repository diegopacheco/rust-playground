#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require curl
cargo test || fail "cargo test failed"

URL="$(service_url app)"
STARTED=""
if ! port_up "$(service_port app)"; then
  "$SCRIPTS/start-all.sh" >/dev/null
  STARTED="yes"
fi

check() {
  local name expected actual
  name="$1"
  expected="$2"
  actual="$3"
  if printf "%s" "$actual" | grep -q -- "$expected"; then
    log "PASS $name"
  else
    [ -n "$STARTED" ] && "$SCRIPTS/stop-all.sh" >/dev/null
    fail "$name expected '$expected'"
  fi
}

check "html home" "<h1>Proa Bookshelf</h1>" "$(curl -s "$URL/")"
check "markdown negotiation" "^# Proa Bookshelf" "$(curl -s -H 'Accept: text/markdown' "$URL/")"
check "explicit markdown" "## \[Continuous Modernization\](/books/3)" "$(curl -s "$URL/index.md")"
check "book page" "Martin Kleppmann" "$(curl -s "$URL/books/2")"
check "book markdown" "Martin Kleppmann - 2017" "$(curl -s "$URL/books/2/md")"
check "unknown book" "404" "$(curl -s -o /dev/null -w '%{http_code}' "$URL/books/99")"

[ -n "$STARTED" ] && "$SCRIPTS/stop-all.sh" >/dev/null
log "all tests passed"
