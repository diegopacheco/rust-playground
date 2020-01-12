#!/bin/bash

docker exec -it postgres sh -c "PGPASSWORD=docker /usr/bin/psql -h 172.17.0.2 -U postgres -d postgres"
