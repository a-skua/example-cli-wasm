SRC := $(shell find src -name "*.rs")
CONFIG := Cargo.toml Cargo.lock

PORT ?= 8080

.PHONY: run
run: target/wasm32-wasip2/release/example-cli.wasm
	wasmtime run -S http $< $(ARGS)

.PHONY: serve
serve: target/wasm32-wasip2/release/example-cli.wasm
	wasmtime run -S http -S inherit-network $< serve -p $(PORT)

.PHONY: build
build: target/wasm32-wasip2/release/example-cli.wasm

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

target/wasm32-wasip2/release/example-cli.wasm: $(CONFIG) $(SRC)
	cargo build --release --target wasm32-wasip2
