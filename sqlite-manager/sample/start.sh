#!/bin/bash

set -e
cd "$(dirname "$0")"

if [ ! -d node_modules ]; then
  bun install
fi

./stop.sh >/dev/null 2>&1 || true

bun run server.js > server.log 2>&1 &
echo $! > .server.pid

bun x vite > vite.log 2>&1 &
echo $! > .vite.pid

for _ in $(seq 1 30); do
  if curl -sf http://localhost:7777/api/stats >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

for _ in $(seq 1 30); do
  if curl -sf http://localhost:5173 >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

if ! curl -sf http://localhost:7777/api/stats >/dev/null 2>&1; then
  echo "api failed to start, see server.log"
  exit 1
fi

if ! curl -sf http://localhost:5173 >/dev/null 2>&1; then
  echo "ui failed to start, see vite.log"
  exit 1
fi

echo "api http://localhost:7777"
echo "ui  http://localhost:5173"
echo "db  $(pwd)/sample.db"
