#!/bin/bash

echo "=== Building usage collector first ==="
cd ../introspector-collector
cargo build --bin working_usage_collector

echo "=== Using collector to compile itself ==="
USAGE_OUTPUT_DIR="./test_usage_data" \
RUSTC="../target/debug/working_usage_collector" \
cargo build --bin working_usage_collector

echo "=== Environment debug output should appear above ==="
