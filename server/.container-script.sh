#!/bin/bash

# This script is mounted into our container and executed.

set -e

# switch to nightly
rustup default nightly

# install musl target
rustup target add x86_64-unknown-linux-musl

# build it
cargo build --release


