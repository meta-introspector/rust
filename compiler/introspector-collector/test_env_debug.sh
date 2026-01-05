#!/bin/bash

echo "=== Building usage collector first ==="
cargo build --bin working_usage_collector

echo "=== Copying binary to safe location ==="
mkdir -p ./collector_store
cp ../../target/debug/working_usage_collector ./collector_store/

echo "=== Cleaning debug target ==="
cargo clean

echo "=== Using collector to compile itself ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="$(pwd)/collector_store/working_usage_collector" \
cargo build --bin working_usage_collector

echo "=== Environment debug output should appear above ==="
