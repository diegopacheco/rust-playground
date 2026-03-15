#!/bin/bash
echo "Generating OpenTelemetry data - hitting /work endpoint continuously..."
echo "Press Ctrl+C to stop"
echo ""
while true; do
  for i in $(seq 1 10); do
    curl -s http://localhost:8080/work > /dev/null &
  done
  wait
  METRICS=$(curl -s http://localhost:8080/metrics 2>/dev/null)
  REQS=$(echo "$METRICS" | grep "http_requests_total{" | awk '{print $2}')
  ERRS=$(echo "$METRICS" | grep "http_errors_total{" | awk '{print $2}')
  echo "requests=$REQS errors=$ERRS"
  sleep 1
done
