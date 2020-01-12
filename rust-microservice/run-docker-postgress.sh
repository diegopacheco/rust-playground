#!/bin/bash

docker run --rm \
  --name postgres -e POSTGRES_PASSWORD=docker \
  -p 5432:5432 -v /tmp/postgress:/var/lib/postgresql/data \
  postgres
