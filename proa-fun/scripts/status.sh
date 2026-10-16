#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

for name in $(service_names); do
  port="$(service_port "$name")"
  pid="$(port_pid "$port")"
  if [ -n "$pid" ]; then
    printf "%-6s %-6s UP   pid %s\n" "$name" "$port" "$pid"
  else
    printf "%-6s %-6s DOWN\n" "$name" "$port"
  fi
done
