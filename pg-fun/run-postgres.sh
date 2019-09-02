#!/bin/bash

docker run --rm --name postgres -e "POSTGRES_PASSWORD=admin" -p 5432:5432 postgres
