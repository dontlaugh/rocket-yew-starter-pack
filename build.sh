#!/bin/bash

set -e

echo "building ui"
pushd ui
# our target is set in Web.toml
cargo web build --release 
popd
echo "ui build complete"

echo "building server"
pushd server
cargo build --release
popd
echo "server build complete"

# TODO build a container that runs a rocket server that serves
# js assets built by cargo-web
