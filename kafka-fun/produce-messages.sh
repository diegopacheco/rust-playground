#!/bin/bash

docker-compose exec kafka1  \
  bash -c "seq 10 | kafka-console-producer --request-required-acks 1 --broker-list localhost:9092 --topic topicrs && echo 'Produced 10 messages.'"
