[packages.example-cli]
path = "target/wasm32-wasip2/release/composed-example-cli.wasm"

[packages.example-cli.run]
wasi = ["http"]
env = ["HOME"]
dirs = ["<?=getenv("HOME")?>/.config/gcloud"]

[packages.example-cli.serve]
wasi = ["cli", "inherit-network"]
