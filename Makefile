SRC := $(shell find src -name "*.rs")
WIT := $(shell find wit -name "*.wit")
CONFIG := Cargo.toml Cargo.lock

GCLOUD_AUTH_WASM_REF := ghcr.io/a-skua/gcloud/auth:0.1.0
GCLOUD_AUTH_WASM := target/wasm32-wasip2/release/gcloud_auth.wasm
GCLOUD_STORAGE_WASM_REF := ghcr.io/a-skua/gcloud/storage:0.1.0
GCLOUD_STORAGE_WASM := target/wasm32-wasip2/release/gcloud_storage.wasm
COMPOSED_STORAGE_WASM := target/wasm32-wasip2/release/composed_storage.wasm

PACKAGE_NAME := example-cli
EXAMPLE_WASM := target/wasm32-wasip2/release/$(PACKAGE_NAME).wasm
COMPOSED_WASM := target/wasm32-wasip2/release/composed.wasm

PORT ?= 8080

.PHONY: run
run: $(COMPOSED_WASM)
	wasmtime run -S http -S inherit-env --dir $(HOME)/.config/gcloud $< $(ARGS)

.PHONY: serve
serve: $(COMPOSED_WASM)
	wasmtime run -S http -S inherit-network $< serve -p $(PORT)

.PHONY: gcloud
gcloud: $(COMPOSED_WASM)
	wasmtime run -S http -S inherit-env --dir $(HOME)/.config/gcloud $< gcloud $(ARGS)

.PHONY: build
build: $(COMPOSED_WASM)

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clean
clean:
	cargo clean

$(GCLOUD_AUTH_WASM):
	wkg oci pull $(GCLOUD_AUTH_WASM_REF) -o $@

$(GCLOUD_STORAGE_WASM):
	wkg oci pull $(GCLOUD_STORAGE_WASM_REF) -o $@

$(EXAMPLE_WASM): $(CONFIG) $(SRC) $(WIT)
	cargo build --release --target wasm32-wasip2

$(COMPOSED_STORAGE_WASM): $(GCLOUD_STORAGE_WASM) $(GCLOUD_AUTH_WASM)
	wac plug $< --plug $(word 2, $^) -o $@

$(COMPOSED_WASM): $(EXAMPLE_WASM) $(COMPOSED_STORAGE_WASM)
	wac plug $< --plug $(word 2, $^) -o $@
