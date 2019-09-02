#!/bin/bash

docker-compose exec kafka1  \
kafka-topics --create --topic topicrs --partitions 1 --replication-factor 1 --if-not-exists --zookeeper zoo1:2181
