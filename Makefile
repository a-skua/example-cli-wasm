SRC := $(shell find src -name "*.rs")
WIT := $(shell find wit -name "*.wit")
CONFIG := Cargo.toml Cargo.lock

GCLOUD_AUTH_WASM_REF := ghcr.io/a-skua/gcloud/auth:0.3.0-component
GCLOUD_AUTH_WASM := target/wasm32-wasip2/release/gcloud_auth.wasm
GCLOUD_STORAGE_WASM_REF := ghcr.io/a-skua/gcloud/storage:0.3.0-component
GCLOUD_STORAGE_WASM := target/wasm32-wasip2/release/gcloud_storage.wasm

PACKAGE_NAME := $(shell cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].name')
EXAMPLE_WASM := target/wasm32-wasip2/release/$(PACKAGE_NAME).wasm
COMPOSED_WASM := target/wasm32-wasip2/release/composed-example-cli.wasm

PORT ?= 8080

.PHONY: run
run: build
	wa run example-cli -h

.PHONY: serve
serve: build
	wa serve example-cli --addr 0.0.0.0:$(PORT)

.PHONY: build
build: $(COMPOSED_WASM)

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: init
init: wasm-pkg-runner.toml

.PHONY: clean
clean:
	cargo clean

wit/deps: wit/*.wit
	wkg wit fetch

%: %.php
	php $< > $@

$(GCLOUD_AUTH_WASM): $(WIT)
	wkg oci pull $(GCLOUD_AUTH_WASM_REF) -o $@

$(GCLOUD_STORAGE_WASM): $(WIT)
	wkg oci pull $(GCLOUD_STORAGE_WASM_REF) -o $@

$(EXAMPLE_WASM): $(CONFIG) $(SRC) $(WIT)
	cargo build --release --target wasm32-wasip2

$(COMPOSED_WASM): compose.wac $(EXAMPLE_WASM) $(GCLOUD_AUTH_WASM) $(GCLOUD_STORAGE_WASM)
	wac compose compose.wac \
		-d "gcloud:auth=$(GCLOUD_AUTH_WASM)" \
		-d "gcloud:storage=$(GCLOUD_STORAGE_WASM)" \
		-d "example:cli=$(EXAMPLE_WASM)" \
		-o $@
