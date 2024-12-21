# helm-wrapper-rs

**Precaution:** experimental crate

Partial helm command subset implementation:

- List releases
- Install chart (through `helm upgrade --install`)
- Uninstall chart

```toml
[dependencies]
helm-wrapper-rs = { git = "https://gitlab.com/weird-crates/helm-wrapper-rs", version = "0.1.0" }
```

## How it works

The create wraps `helm` executable and parse results.

## Run integration tests

What tests do:

- Install whoami helm chart
- Get information about installed charts (helm releases)
-

Kubernetes cluster is required. You can use K3s:

```shell
curl -sfL https://get.k3s.io | INSTALL_K3S_EXEC="--disable=traefik --flannel-iface=wlan0" sh -
chown $USER: /etc/rancher/k3s/k3s.yaml
export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
```

Run tests:

```shell
cargo test
```

## RoadMap

- Strict type checking with nutype
