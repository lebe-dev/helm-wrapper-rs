# helm-wrapper-rs

Helm wrapper library for Rust.

Commands supported:

- List releases
- Install chart (through `helm upgrade --install`)
- Uninstall chart
- Safety mode (by default). Don't log sensitive data.

## Getting started

```toml
[dependencies]
helm-wrapper-rs = "0.2.0"
```

```rust
use crate::nonblocking::DefaultHelmExecutor;

let helm_executor = DefaultHelmExecutor::new();

let releases = helm_executor.list_releases();

println!("{}", releases);
```

## Features

- `blocking` (default)
- `nonblocking`

## Mock

Add `blocking-mock` or `nonblocking-mock` features:

```toml
helm-wrapper-rs = { version = "0.2.0", features=["blocking-mock"] }
```

Then use `MockHelmExecutor`.

## Run integration tests

What tests do:

- Install [whoami](https://github.com/traefik/whoami) helm chart
- Get information about installed charts (helm releases)
- Uninstall whoami helm chart

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
