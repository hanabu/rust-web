# Yew Hello World sample

## Setup

We use `wasm-pack` to build WASM. Setup it with `cargo install`

```
cargo install wasm-pack
```

## Build

Run [build.sh](build.sh).

```
build.sh

[INFO]: Compiling to Wasm...
   Compiling ...
   Compiling yew-helloworld v0.1.0
    Finished release [optimized] target(s) in 1m 30s
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: :-) Done in 1m 30s
[INFO]: :-) Your wasm pkg is ready to publish at yew-helloworld/pkg.

main.js → pkg/bundle.js...

```

You get `pkg/bundle.js` and `pkg/yew\_helloworld\_bg.wasm`

## Run

```
run.sh
```

Then, open [http://localhost:1111](http://localhost:1111) on your browser.

