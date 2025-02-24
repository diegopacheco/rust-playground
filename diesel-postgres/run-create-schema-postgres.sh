#!/bin/bash

echo "CREATE DATABASE users;" | docker run --network host -i postgres psql -h localhost -U postgres