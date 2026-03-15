#!/bin/bash
podman-compose up -d
echo "Prometheus: http://localhost:9090"
echo "Grafana:    http://localhost:3000 (no login needed)"
echo "Dashboard:  http://localhost:3000/d/rust-otel-dashboard"
