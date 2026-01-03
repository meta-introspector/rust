#!/bin/bash

# Build rustc_driver using our introspector-collector
echo "=== Building rustc_driver with introspector-collector ==="

# Build our collector first
cd ../introspector-collector
cargo build --bin working_usage_collector
cd ../rustc_driver

# Use our collector as RUSTC to build current directory
CFG_RELEASE_CHANNEL=dev RUSTC_INSTALL_BINDIR=/usr/bin RUSTC=../../target/debug/working_usage_collector cargo build

echo "=== Results ==="
ls -la usage_data/
