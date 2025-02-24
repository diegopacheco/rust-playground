#!/bin/bash

curl -i http://localhost:8080/cats
curl -i -X PUT \
  -H "Content-Type: application/json" \
  -d '{"name":"Tiger","cat_type":"wild","fun_fact":"hunts at night"}' \
  http://localhost:8080/cats