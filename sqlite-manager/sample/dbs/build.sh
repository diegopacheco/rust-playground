#!/bin/bash

set -e
cd "$(dirname "$0")"

for sql in *.sql; do
  db="${sql%.sql}.db"
  rm -f "$db" "$db-wal" "$db-shm"
  sqlite3 "$db" < "$sql"
  echo "built $db"
done
