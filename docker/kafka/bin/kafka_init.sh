#!/bin/bash

TOPICS = []
BROKER = "kafka:29092"

echo "Waiting until Kafka broker $BROKER is available..."
kafka-topics --bootstrap-server "$BROKER" --list

echo "Creating Kafka topics..."
for topic in "${TOPICS[@]}"; do
  echo "Creating Kafka topic $topic"
  kafka-topics --bootstrap-server "$BROKER" --create --if-not-exists --topic "$topic" --replication-factor 1 --partitions 1
done

echo "Successfully created the following topics:"
kafka-topics --bootstrap-server "$BROKER" --list
