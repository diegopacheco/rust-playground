#!/bin/bash
cargo build 2>&1
cargo run &
APP_PID=$!

while ! curl -s http://localhost:8080/health > /dev/null 2>&1; do
  sleep 1
done
echo "App is up at http://localhost:8080"
echo "Metrics at http://localhost:8080/metrics"

echo ""
echo "Sending traffic to /work ..."
for i in $(seq 1 20); do
  curl -s http://localhost:8080/work | head -c 80
  echo ""
done

echo ""
echo "Prometheus metrics output:"
curl -s http://localhost:8080/metrics

kill $APP_PID 2>/dev/null
wait $APP_PID 2>/dev/null
