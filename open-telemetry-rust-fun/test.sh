#!/bin/bash
echo "Testing /health ..."
curl -s http://localhost:8080/health
echo ""

echo "Testing /work ..."
curl -s http://localhost:8080/work
echo ""

echo "Testing /metrics ..."
curl -s http://localhost:8080/metrics | head -30
echo ""

echo "Testing Prometheus ..."
curl -s http://localhost:9090/api/v1/targets | head -c 200
echo ""

echo "Testing Grafana ..."
curl -s http://localhost:3000/api/health
echo ""
