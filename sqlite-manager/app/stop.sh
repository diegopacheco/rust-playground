#!/bin/bash

cd "$(dirname "$0")"

for name in server vite; do
  file=".${name}.pid"
  if [ -f "$file" ]; then
    pid=$(cat "$file")
    if kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
      for _ in $(seq 1 10); do
        if ! kill -0 "$pid" 2>/dev/null; then
          break
        fi
        sleep 1
      done
      kill -9 "$pid" 2>/dev/null || true
    fi
    rm -f "$file"
  fi
done

echo "stopped"
