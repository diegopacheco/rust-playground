#!/usr/bin/env bash
set -euo pipefail
. "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

require_cargo

log "toolchain: $(rustc --version)"
log "cargo:     $(cargo --version)"

for component in rustfmt clippy; do
  if rustup component list --installed 2>/dev/null | grep -q "^$component"; then
    log "component $component already installed"
  else
    log "installing component $component"
    rustup component add "$component"
  fi
done

if have cargo-pretty; then
  log "cargo-pretty already installed"
else
  log "installing cargo-pretty"
  cargo install cargo-pretty-build
fi

log "fetching dependencies"
cargo fetch --quiet

log "setup complete"
