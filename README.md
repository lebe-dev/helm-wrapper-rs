# helm-wrapper-rs

Helm wrapper library for Rust.

Commands supported:

- List releases
- Install chart (through `helm upgrade --install`)
- Uninstall chart
- Safety mode (by default). Don't log sensitive data.

## Getting started

To use `helm-wrapper-rs`, add it to your `Cargo.toml`:

```toml
[dependencies]
helm-wrapper-rs = "0.4.1"
```

## Features

- `blocking` (default)
- `nonblocking`

## Examples

Check [examples](examples) directory for usage examples.

## Mock

Add `blocking-mock` or `nonblocking-mock` features:

```toml
helm-wrapper-rs = { version = "0.4.1", features = ["blocking-mock"] }
```

Then use `MockHelmExecutor`.

## Development

First of all I suggest you to install `mold` linker for faster compilation time.

### Run integration tests

Kubernetes cluster is required. You can use K3s:

```bash
curl -sfL https://get.k3s.io | sh -
chown $USER: /etc/rancher/k3s/k3s.yaml
chmod g-r /etc/rancher/k3s/k3s.yaml

export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
```

Run tests:

```bash
cargo test --no-default-features --features "blocking"
cargo test --no-default-features --features "nonblocking"
```

## RoadMap

- Strict type checking with nutype
