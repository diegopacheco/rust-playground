#!/bin/bash

docker run --rm --name postgres --network host -e POSTGRES_PASSWORD=root -d postgres