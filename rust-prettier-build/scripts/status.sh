#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

report() {
  if [ -n "$2" ]; then
    printf "%-16s %-6s %s\n" "$1" "OK" "$2"
  else
    printf "%-16s %-6s %s\n" "$1" "MISSING" "-"
  fi
}

printf "%-16s %-6s %s\n" "COMPONENT" "STATE" "DETAIL"

if have cargo; then report "cargo" "$(cargo --version)"; else report "cargo" ""; fi
if have rustc; then report "rustc" "$(rustc --version)"; else report "rustc" ""; fi
if have cargo-pretty; then report "cargo-pretty" "installed"; else report "cargo-pretty" ""; fi

if [ -f "$(debug_binary)" ]; then report "debug build" "$(debug_binary)"; else report "debug build" ""; fi
if [ -f "$(release_binary)" ]; then report "release build" "$(release_binary)"; else report "release build" ""; fi

TESTS="$(grep -rc "fn .*(" --include=*.rs src tests 2>/dev/null | awk -F: '{s+=$2} END {print s+0}')"
report "sources" "$(find src tests -name '*.rs' | wc -l | tr -d ' ') rust files, $TESTS functions"
