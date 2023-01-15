# Yew Hello World sample

## Setup

We use `wasm-pack` to build WASM. Setup it with `cargo install`

```console
$ cargo install wasm-pack
```

## Build

Run [build.sh](build.sh).

```console
$ build.sh

[INFO]: Compiling to Wasm...
   Compiling ...
   Compiling yew-helloworld v0.1.0
    Finished release [optimized] target(s) in 1m 30s

```

You get `legacy/yew-helloworld.js` and `legacy/yew-helloworld\_bg.wasm`

## Run

```console
$ run.sh
Serving HTTP on 0.0.0.0 port 1111 (http://0.0.0.0:1111/) ...
```

Then, open [http://localhost:1111](http://localhost:1111) on your browser.
