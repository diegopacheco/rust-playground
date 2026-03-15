#!/bin/bash
cargo build 2>&1

podman-compose up -d
while ! curl -s http://localhost:9090/-/ready > /dev/null 2>&1; do
  sleep 1
done
echo "Prometheus is ready"

while ! curl -s http://localhost:3000/api/health > /dev/null 2>&1; do
  sleep 1
done
echo "Grafana is ready"

cargo run &
APP_PID=$!
while ! curl -s http://localhost:8080/health > /dev/null 2>&1; do
  sleep 1
done
echo "App is up at http://localhost:8080"

echo ""
echo "Sending traffic to /work ..."
for i in $(seq 1 30); do
  curl -s http://localhost:8080/work | head -c 80
  echo ""
done

echo ""
echo "Prometheus metrics:"
curl -s http://localhost:8080/metrics
echo ""
echo "=========================================="
echo "Grafana Dashboard: http://localhost:3000/d/rust-otel-dashboard"
echo "Prometheus:        http://localhost:9090"
echo "App:               http://localhost:8080"
echo "=========================================="
echo "Press Ctrl+C to stop..."
wait $APP_PID
