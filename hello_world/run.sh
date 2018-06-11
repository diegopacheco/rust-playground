#!/bin/bash

rustc src/main.rs
./main
rm -rf main
