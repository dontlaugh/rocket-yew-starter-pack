#!/bin/bash

# build frontend assets and put them in a place the Rocket server
# expects

echo "building ui"
pushd ui
cargo web build --release
popd
echo "ui build complete"


cp ui/target/asmjs-unknown-emscripten/release/ui.js server/static/ui.js
cp ui/static/styles.css server/static/styles.css

(
  cd server
  cargo run
)
