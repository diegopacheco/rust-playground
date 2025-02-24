#!/bin/bash

docker stop mysql_test
docker rm mysql_test
echo "DONE."