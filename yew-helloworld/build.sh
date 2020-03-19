#!/bin/sh

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir pkg --no-typescript \
             target/wasm32-unknown-unknown/release/yew_helloworld.wasm

wasm-bindgen --target no-modules --out-dir legacy --no-typescript \
             target/wasm32-unknown-unknown/release/yew_helloworld.wasm

# wasm-pack build --target no-modules --no-typescript
