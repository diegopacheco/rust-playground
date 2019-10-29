#!/bin/bash

rustup default nightly
rustup override set nightly
rustup update && cargo update