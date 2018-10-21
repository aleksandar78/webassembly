#!/bin/bash

# Build wasm module
cargo build --target wasm32-unknown-unknown --release

# Compress wasm
wasm-gc target/wasm32-unknown-unknown/release/utils.wasm -o utils.gc.wasm

exit 0