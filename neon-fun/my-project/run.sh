#!/bin/bash

neon build --release
echo "require('.')" | node
