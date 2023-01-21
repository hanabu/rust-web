# Rust Web sample

## Yew

[Yew](https://github.com/yewstack/yew) is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.

### Yew samples of struct component

- [helloworld](yew-struct-components/helloworld)
- [counter](yew-struct-components/counter)
- [iterator](yew-struct-components/iterator)
- [iterator](yew-struct-components/fetch)

## Build

Use [Trunk](https://github.com/thedodd/trunk) to build app.

### Install trunk

```console
$ cargo install trunk
```

### Build and run Yew application

```console
$ cd yew-struct-components/helloworld
$ trunk serve
```

You see app on http://127.0.0.1:8080

### Release build

```console
$ cd yew-struct-components/helloworld
$ trunk build --release
```

You can deploy `yew-struct-components/helloworld/dist` directory to the web server.

