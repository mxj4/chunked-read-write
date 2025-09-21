#!/usr/bin/env bash
set -euo pipefail

# You need to install Clang and possibly other build tools for cross-compiling C/C++ code to WebAssembly.
sudo apt update
sudo apt install clang -y

# Install Dioxus CLI and add wasm target. Assumes rustup and cargo are already present
# in the base devcontainer image (official Rust devcontainer includes rustup).

echo "Adding wasm32-unknown-unknown target (if not already added)..."
rustup target add wasm32-unknown-unknown || true

echo "Installing or updating dioxus-cli via cargo..."
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# Try to install; if already installed, attempt a reinstall/force to update.
if ! cargo install --list | grep -q "dioxus-cli"; then
    # If already present, reinstall to ensure latest. --force is used to update.
    cargo binstall dioxus-cli@0.7.0-rc.0 --no-confirm --locked --force || true
fi

echo "Dioxus install script finished."
