#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target no-modules --out-dir pkg --no-typescript \
             target/wasm32-unknown-unknown/release/yew_counter.wasm

# wasm-pack build --target no-modules --no-typescript
