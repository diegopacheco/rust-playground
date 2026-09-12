#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

if [ ! -t 1 ]; then
  fail "run.sh needs an interactive terminal"
fi

cargo_verb run --release -- "$@"
