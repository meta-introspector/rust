#!/bin/bash

# Build rustc_driver using our usage_collector as the rustc driver
echo "=== Building rustc_driver with usage_collector ==="

# Build our collector first
rustc usage_collector.rs -o usage_collector

# Use our collector as RUSTC to build current directory
RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver/usage_collector cargo build

echo "=== Results ==="
ls -la usage_data/
