SRC := $(shell find src -name "*.rs")
CONFIG := Cargo.toml Cargo.lock

.PHONY: run
run: target/wasm32-wasip2/release/example-cli.wasm
	wasmtime run target/wasm32-wasip2/release/example-cli.wasm

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

target/wasm32-wasip2/release/example-cli.wasm: $(CONFIG) $(SRC)
	cargo build --release --target wasm32-wasip2
