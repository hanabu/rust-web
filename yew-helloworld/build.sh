#!/bin/sh

if [ ! -f ./node_modules/rollup/dist/bin/rollup ]; then
  npm install rollup
fi

wasm-pack build --target web && \
./node_modules/rollup/dist/bin/rollup -i main.js -f iife -o pkg/bundle.js
