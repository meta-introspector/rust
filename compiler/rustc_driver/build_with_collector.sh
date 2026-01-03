#!/bin/bash

# Build rustc_driver using our introspector-collector
echo "=== Building rustc_driver with introspector-collector ==="

# Build our collector first
cd .. && cargo build --bin working_usage_collector
cd rustc_driver

# Use our collector as RUSTC to build ALL crates
CFG_RELEASE_CHANNEL=dev RUSTC_INSTALL_BINDIR=/usr/bin RUSTC=../../target/debug/working_usage_collector cargo build --workspace --all-targets

echo "=== Results ==="
ls -la ../../usage_data/ | wc -l
echo "Files created in ../../usage_data/"
