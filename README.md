# Example Wasm (CLI)

## build

```sh
make build
```

or

```sh
cargo build --release --target wasm32-wasip2
```

## examples

### CLI

```sh
make run ARGS='get -h'
```

or

```sh
wasmtime run -S http target/wasm32-wasip2/release/example-cli.wasm get -h
```

### HTTP Server

```sh
make serve PORT=8080
```

or

```sh
wasmtime run -S http -S inherit-network target/wasm32-wasip2/release/example-cli.wasm serve -p 8080
```
