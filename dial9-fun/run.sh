#!/bin/bash

mkdir -p target/traces
cargo build 2>&1
cargo run 2>&1
