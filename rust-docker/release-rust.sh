#!/bin/bash

here=$(pwd)
builder="docker run --rm -it -v $here:/home/rust/src ekidd/rust-musl-builder"
$builder cargo build --release
ls -lh target/x86_64-unknown-linux-musl/release/main