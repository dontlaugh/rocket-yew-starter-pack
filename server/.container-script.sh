#!/bin/bash

set -e

# Meant to be run as an argument to our container

# switch to nightly
rustup default nightly

# install musl target
rustup target add x86_64-unknown-linux-musl

# build it
cargo build --release


