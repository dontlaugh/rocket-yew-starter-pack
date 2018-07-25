#!/bin/bash

set -e

# build frontend assets and put them in a place the Rocket server
# expects


echo "building ui"
pushd ui
cargo web build --release --target=wasm32-unknown-unknown
popd
echo "ui build complete"

cp ui/target/wasm32-unknown-unknown/release/ui.js server/static/ui.js
cp ui/target/wasm32-unknown-unknown/release/ui.wasm server/static/ui.wasm
cp ui/static/styles.css server/static/styles.css

(
  echo "running server"
  cd server
  cargo run --release
)


