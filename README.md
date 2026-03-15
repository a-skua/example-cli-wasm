# Example Wasm (CLI)

## Setup

```sh
make init
```

## Build

```sh
make build
```

## Examples

### CLI

```sh
wa run example-cli hello
```

```sh
wa run example-cli get -h
```

### HTTP Server

```sh
make serve PORT=8080
```

or

```sh
wa serve example-cli --addr 0.0.0.0:8080
```

### Gcloud

```sh
wa run example-cli gcloud auth token get --scopes https://www.googleapis.com/auth/cloud-platform
```

```sh
wa run example-cli gcloud storage buckets list --project <project-id>
```
