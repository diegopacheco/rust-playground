#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

port_up "$(service_port app)" || fail "app is not running, run ./scripts/start-all.sh"
open_url "$(service_url app)"
