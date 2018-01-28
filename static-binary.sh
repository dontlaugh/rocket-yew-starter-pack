#!/bin/bash

# Use Eric Kidd's magical docker image
# https://github.com/emk/rust-musl-builder

(
  cd server
  docker run --rm -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder ./.container-script.sh
)
