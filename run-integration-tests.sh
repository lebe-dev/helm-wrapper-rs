#!/bin/bash

# Install `whoami` before running tests:
# helm repo add cowboysysop https://cowboysysop.github.io/charts/
# helm repo update
# helm upgrade --install -n whoami whoami cowboysysop/whoami

export KUBECONFIG=~/.kube/dev.kubeconfig

cargo test --no-default-features --features "blocking"
cargo test --no-default-features --features "nonblocking"
