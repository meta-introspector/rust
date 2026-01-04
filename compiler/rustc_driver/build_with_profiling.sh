#!/bin/bash

# Build rustc_driver with comprehensive profiling
echo "=== Building rustc_driver with comprehensive profiling ==="

# Clean first
echo "Cleaning build artifacts..."
cargo clean

# Build with self-profiling enabled
echo "Building with self-profile..."
rustc -Z self-profile=rustc_driver_profile --crate-type=bin src/main.rs -o rustc_driver_traced 2>&1

# Build working_usage_collector first
echo "Building working_usage_collector..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/introspector-collector && cargo build --bin working_usage_collector

# Build with usage collector
echo "Building with usage collector..."
cd /mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/rustc_driver
export CFG_RELEASE_CHANNEL=dev
USAGE_OUTPUT_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/usage_data" CFG_RELEASE_CHANNEL=dev RUSTC_INSTALL_BINDIR=/usr/bin RUSTC=/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/target/debug/working_usage_collector cargo build --workspace --all-targets

# Generate Chrome profiler data
echo "Generating Chrome profiler data..."
if [ -d "rustc_driver_profile" ]; then
    /mnt/data1/nix/vendor/rust/cargo2nix/submodules/measureme/target/debug/crox --dir rustc_driver_profile
    echo "Chrome profiler data generated: chrome_profiler.json"
fi

echo "=== Profiling Results ==="
echo "Self-profile data:"
ls -la rustc_driver_profile/ 2>/dev/null | wc -l
echo "Usage data files:"
ls -la ../../usage_data/ | wc -l
echo "Chrome profiler size:"
ls -lh chrome_profiler.json 2>/dev/null

echo "=== Analysis Ready ==="
echo "View chrome_profiler.json in Chrome DevTools Performance tab"
echo "Usage data available in ../../usage_data/"
