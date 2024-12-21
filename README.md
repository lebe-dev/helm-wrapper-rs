# helm-wrapper-rs

**Precaution:** experimental crate

Partial helm command subset implementation.

Features:

- List releases
- Install chart (through `helm upgrade --install`)
- Uninstall chart
- Safety mode (by default). Don't log sensitive data.

## Getting started

```toml
[dependencies]
helm-wrapper-rs = { git = "https://gitlab.com/weird-crates/helm-wrapper-rs", version = "0.1.0" }
```

```rust
let helm_executor = DefaultHelmExecutor::new();

let releases = helm_executor.list_releases();

println!("{}", releases);
```

## How it works

The create wraps `helm` executable and parse results.

## Run integration tests

What tests do:

- Install [whoami](https://github.com/traefik/whoami) helm chart
- Get information about installed charts (helm releases)
- Uninstall whoami helm chart

Kubernetes cluster is required. You can use K3s:

```shell
curl -sfL https://get.k3s.io | sh -
chown $USER: /etc/rancher/k3s/k3s.yaml
chmod g-r /etc/rancher/k3s/k3s.yaml

export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
```

Run tests:

```shell
cargo test
```

## RoadMap

- Strict type checking with nutype
