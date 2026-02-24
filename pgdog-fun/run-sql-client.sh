#!/bin/bash
echo "Password: pgdog"
podman exec -it -e PGPASSWORD=pgdog pgdog-fun_postgres_1 psql -h pgdog -p 6432 -U pgdog -d pgdog
