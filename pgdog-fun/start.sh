#!/bin/bash
podman-compose up -d
echo "Waiting for postgres to be ready..."
while ! podman exec pgdog-fun_postgres_1 pg_isready -U pgdog > /dev/null 2>&1; do
  sleep 1
done
echo "Postgres is ready."
echo "Waiting for pgdog proxy to be ready..."
while ! nc -z localhost 6432 > /dev/null 2>&1; do
  sleep 1
done
echo "pgdog proxy is ready on port 6432."
podman-compose ps
