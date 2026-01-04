#!/bin/bash

# Build rustc_driver using our introspector-collector
echo "=== Building rustc_driver with introspector-collector ==="

# Use our collector as RUSTC to build ALL crates
USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" CFG_RELEASE_CHANNEL=dev RUSTC_INSTALL_BINDIR=/usr/bin RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector cargo build --workspace --all-targets

echo "=== Results ==="
ls -la ../../usage_data/ | wc -l
echo "Files created in /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_data/"
ls /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/usage_data/*.json 2>/dev/null | tail -5
