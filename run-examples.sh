#!/bin/bash

# Install `whoami` before running tests:
# helm repo add cowboysysop https://cowboysysop.github.io/charts/
# helm repo update
# helm upgrade --install -n whoami whoami cowboysysop/whoami

export KUBECONFIG=~/.kube/dev.kubeconfig

cargo run --example nonblocking_list --features "nonblocking"
cargo run --example blocking_list --features "blocking"
#!/bin/bash

# Ensure KUBECONFIG is set, similar to integration tests
# You might need to adjust this path to your actual kubeconfig file
export KUBECONFIG=${KUBECONFIG:-~/.kube/config}
# If using k3s as per README development section:
# export KUBECONFIG=${KUBECONFIG:-/etc/rancher/k3s/k3s.yaml}

echo "Using KUBECONFIG=$KUBECONFIG"
echo

echo "Running blocking_list example..."
cargo run --example blocking_list --features blocking
echo

echo "Running nonblocking_list example..."
cargo run --example nonblocking_list --features nonblocking
echo

echo "Running blocking_install_or_upgrade example..."
cargo run --example blocking_install_or_upgrade --features blocking
echo

echo "Running nonblocking_install_or_upgrade example..."
cargo run --example nonblocking_install_or_upgrade --features nonblocking
echo

echo "Running blocking_uninstall example..."
cargo run --example blocking_uninstall --features blocking
echo

echo "Running nonblocking_uninstall example..."
cargo run --example nonblocking_uninstall --features nonblocking
echo
