#!/bin/bash

docker-compose exec kafka1  \
  kafka-console-consumer --bootstrap-server localhost:9092 --topic topicrs --from-beginning --max-messages 100
