#!/bin/bash

echo "building ui"
pushd ui
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
