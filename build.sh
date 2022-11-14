#!/bin/zsh

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/wasm_tracer.wasm

cargo build --target $TARGET --release

wasm-snip --snip-rust-fmt-code \
          --snip-rust-panicking-code \
          -o $BINARY \
          $BINARY
wasm-strip $BINARY

mkdir -p www/
wasm-opt -o www/wasm_tracer.wasm -Oz $BINARY
ls -lh www/wasm_tracer.wasm
