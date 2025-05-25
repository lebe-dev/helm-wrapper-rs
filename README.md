# helm-wrapper-rs

Helm wrapper library for Rust.

Commands supported:

- List releases
- Install chart (through `helm upgrade --install`)
- Uninstall chart
- Safety mode (by default). Don't log sensitive data.

## Getting started

To use `helm-wrapper-rs`, add it to your `Cargo.toml`. For example, to use the `nonblocking` feature:

```toml
[dependencies]
helm-wrapper-rs = { version = "0.4.1", features = ["nonblocking"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
non-blank-string-rs = "1.0.4"
```

You can find an example of how to list Helm releases using the nonblocking API in `examples/nonblocking_list.rs`.

To run all examples, ensure your `KUBECONFIG` environment variable is set correctly and then execute:
```bash
./run-examples.sh
```
Make sure the script is executable: `chmod +x run-examples.sh`.

## Features

- `blocking` (default) - See example below for blocking usage.
- `nonblocking`

### Blocking Feature Example

To use the `blocking` feature (which is enabled by default if no other feature is specified):

```toml
[dependencies]
helm-wrapper-rs = { version = "0.4.1", features = ["blocking"] }
non-blank-string-rs = "1.0.4"
```

You can find an example of how to list Helm releases using the blocking API in `examples/blocking_list.rs`.

To run all examples, ensure your `KUBECONFIG` environment variable is set correctly and then execute:
```bash
./run-examples.sh
```
Make sure the script is executable: `chmod +x run-examples.sh`.

## Mock

Add `blocking-mock` or `nonblocking-mock` features:

```toml
helm-wrapper-rs = { version = "0.4.1", features=["blocking-mock"] }
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
